mod options;

use crate::{
    copiee::copy_files, exec::exec_batch_ffmpeg, progress::JobInfo, Error, ProgressSystem, Result,
};
pub use lib_sorter::{Bucket, Sinker};
use lib_utils::file::FileExt;
pub use options::{
    ArgsType, AudioCodec, ConverterOptions, FfmpegOptions, HwAccel, OutputExtension, PictureFormat,
    Resolution, VideoCodec,
};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::{sync::watch::Receiver as WatchReceiver, sync::RwLock, task::JoinSet};

#[derive(Default, Debug)]
pub enum State {
    #[default]
    Idle,
    TaskAvailable,
}

#[derive(Default, Debug)]
pub struct Converter {
    options: Option<Arc<ConverterOptions>>,
    progress_system: Option<Arc<RwLock<ProgressSystem>>>,
    buckets: Option<Vec<(Arc<str>, Bucket)>>,
    state: State,
    stop_signal: Option<WatchReceiver<bool>>,
}

impl Converter {
    pub fn new(stop_signal: WatchReceiver<bool>) -> Self {
        Self {
            stop_signal: Some(stop_signal),
            ..Default::default()
        }
    }

    pub fn new_with_progress_tracker(
        stop_signal: WatchReceiver<bool>,
        progress_system: Arc<RwLock<ProgressSystem>>,
    ) -> Self {
        let mut a = Self::new(stop_signal);
        a.progress_system = Some(progress_system);
        a
    }

    pub fn reset(&mut self) {
        // Clear the existing state and set to default
        self.options = None;
        self.state = State::Idle;
        self.stop_signal = None;
    }
    pub async fn prepare_task(&mut self, options: Arc<ConverterOptions>) -> Result<()> {
        if !options.input_dir.clone().exists() {
            return Err(Error::NotExistanceInputOutputDir);
        }

        // Read the directory
        let entries = fs::read_dir(options.input_dir.clone())
            .map_err(|err| Error::ReadDirError(err.to_string()))?;

        let all_entries_path: Vec<PathBuf> = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .collect();

        // Let it sink
        let buckets: Vec<(Arc<str>, Bucket)> =
            Sinker::sink(all_entries_path, options.need_sorting)?
                .into_iter()
                .collect();

        if let Some(progress_system) = &self.progress_system {
            for (title, bucket) in buckets.iter() {
                let job_info = JobInfo::new(
                    title.clone(),
                    bucket.video_files().len(),
                    bucket.xml_files().len(),
                );
                progress_system
                    .read()
                    .await
                    .create_tracker(&job_info)
                    .await?;
            }
        }

        self.buckets = Some(buckets);
        self.options = Some(options);
        self.state = State::TaskAvailable;
        Ok(())
    }

    pub async fn start_conversion(
        &mut self,
        ffmpeg_executable: Option<&'static PathBuf>,
    ) -> Result<()> {
        let converter_opts = format!(
            "Converting started with options : {}",
            serde_json::to_string(&self.options.as_deref().unwrap()).unwrap()
        );

        tracing::info!(converter_opts);
        // Ensure that a task is available before proceeding
        if !matches!(self.state, State::TaskAvailable) {
            return Err(Error::ConverterHasNoTaskAvailable);
        }

        let Some(buckets) = self.buckets.take() else {
            return Err(Error::ConverterHasNoTaskAvailable);
        };

        let max_concurrency = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(2);

        let mut join_set = JoinSet::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrency));
        for (name, bucket) in buckets.into_iter() {
            let semaphore = semaphore.clone();
            let progress_system = self.progress_system.clone();
            let stop_signal = self.stop_signal.clone().unwrap();
            let options = self.options.clone().unwrap();

            let (folder_name, xml, video) = bucket.into_parts();

            join_set.spawn(async move {
                tracing::info!("Spawning new thread for bucket : {}", name);

                let permit = semaphore.acquire_owned().await.unwrap();
                let _ = Converter::convert(
                    options.as_ref(),
                    folder_name,
                    xml,
                    video,
                    stop_signal,
                    ffmpeg_executable,
                    progress_system,
                )
                .await;

                drop(permit);
            });
        }

        // Wait for all tasks to complete
        join_set.join_all().await;

        // Reset internal state after spawning all tasks
        self.state = State::Idle;
        self.stop_signal = None;

        Ok(())
    }

    async fn convert(
        options: &ConverterOptions,
        name: Arc<str>,
        xml_files: Arc<[PathBuf]>,
        video_files: Arc<[FileExt]>,
        stop_signal: WatchReceiver<bool>,
        ffmpeg_executable: Option<&'static PathBuf>,
        progress_system: Option<Arc<RwLock<ProgressSystem>>>,
    ) -> Result<()> {
        tracing::info!("Converting files in bucket : {}", name.clone());

        // Create the output directory
        let mut output = options.output_dir.clone();
        let folder_name = name.clone();
        let mut name = name.to_string();
        name.push_str(" 原");
        output.push(name);

        if let Err(e) = create_directory_with_permissions(&output) {
            tracing::error!("Failed to create directory {:?}: {:?}", output, e);
            return Err(e);
        }

        // Create the XML directory inside the output directory
        let mut xml_dir = output.clone();
        xml_dir.push("xml");
        if let Err(e) = create_directory_with_permissions(&xml_dir) {
            tracing::error!("Failed to create directory {:?}: {:?}", xml_dir, e);
            return Err(e);
        }

        // Copy the sorted XML files into the XML directory
        if let Err(e) = copy_files(
            xml_files,
            &xml_dir,
            folder_name.clone(),
            progress_system.clone(),
        )
        .await
        {
            tracing::error!("Failed to copy files: {:?}", e);
            return Err(e);
        }

        tracing::info!("done copying files in bucket : {}", folder_name);

        // Execute the FFmpeg batch processing with a stop signal
        exec_batch_ffmpeg(
            video_files,
            output,
            options.ffmpeg_options,
            stop_signal.clone(),
            ffmpeg_executable,
            progress_system.clone(),
            folder_name.clone(),
        )
        .await?;

        tracing::info!("done converting files in bucket : {}", folder_name);

        if let Some(progress_system) = progress_system {
            progress_system.read().await.done(folder_name).await?
        }

        Ok(())
    }
}

#[allow(clippy::permissions_set_readonly_false)]
fn create_directory_with_permissions(path: &Path) -> Result<()> {
    // Create directory and set permissions
    fs::create_dir_all(path).map_err(|err| Error::CouldNotCreateDir(err.to_string()))?;

    // Set directory permissions to writable (non-read-only)
    let mut permissions = fs::metadata(path)
        .map_err(|err| Error::CouldNotCreateDir(err.to_string()))?
        .permissions();
    permissions.set_readonly(false);
    fs::set_permissions(path, permissions)
        .map_err(|err| Error::CouldNotCreateDir(err.to_string()))?;
    Ok(())
}
