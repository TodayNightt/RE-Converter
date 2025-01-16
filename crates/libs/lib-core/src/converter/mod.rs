mod options;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

pub use lib_sorter::{Bucket, Sinker};
// use no_deadlocks::prelude::RwLock;
use tokio::sync::{broadcast, RwLock};

use crate::{
    copiee::copy_files, exec::exec_batch_ffmpeg, state::State, Error, ProgressTracker, Result,
};
pub use options::{
    ArgsType, AudioCodec, ConverterOptions, FfmpegOptions, HwAccel, OutputExtension, PictureFormat,
    Resolution, VideoCodec,
};

pub struct Converter {
    options: Option<Arc<ConverterOptions>>,
    progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>>,
    trackers_to_update: Option<Arc<Vec<(String, Arc<ProgressTracker>, Bucket)>>>,
    state: State,
    stop_signal: Option<broadcast::Sender<()>>,
}

impl Converter {
    pub fn new(progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>>) -> Self {
        Self {
            options: None,
            progress_trackers,
            trackers_to_update: None,
            state: State::Idle,
            stop_signal: None,
        }
    }

    pub fn reset(&mut self) {
        // Clear the existing state and set to default
        self.options = None;
        self.trackers_to_update = None;
        self.state = State::Idle;
        self.stop_signal = None;
        // The `progress_trackers` remain unchanged because it's shared state
    }
    pub async fn prepare_task(
        &mut self,
        options: Arc<ConverterOptions>,
    ) -> Result<broadcast::Sender<()>> {
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
        let buckets = Sinker::sink(all_entries_path, options.need_sorting)?;

        // Collect trackers to be updated outside the parallel loop
        let trackers_to_update: Arc<Vec<_>> = Arc::new(
            buckets
                .into_iter()
                .map(|(name, bucket)| {
                    let tracker = Arc::new(ProgressTracker::new(
                        bucket.xml_files().len() as u32,
                        bucket.video_files().len() as u32,
                        name.clone(),
                    ));
                    (name, tracker, bucket)
                })
                .collect(),
        );

        // Update the trackers outside the parallel loop
        let progress_trackers_clone = Arc::clone(&self.progress_trackers);
        let trackers_to_update_clone = Arc::clone(&trackers_to_update);
        async move {
            let mut hm = progress_trackers_clone.write().await;
            for (name, tracker, _) in trackers_to_update_clone.as_ref() {
                hm.insert(name.clone(), tracker.clone());
            }
        }
        .await;

        let (stop_tx, _) = broadcast::channel(32);

        self.options = Some(options);
        self.trackers_to_update = Some(trackers_to_update);
        self.state = State::TaskAvailable;
        self.stop_signal = Some(stop_tx.clone());
        Ok(stop_tx)
    }

    pub fn start_conversion(&mut self) -> Result<()> {
        // Ensure that a task is available before proceeding
        if !matches!(self.state, State::TaskAvailable) {
            return Err(Error::ConverterHasNoTaskAvailable);
        }

        let trackers_to_update: Vec<(String, Arc<ProgressTracker>, Bucket)> = self
            .trackers_to_update
            .take()
            .ok_or(Error::ConverterHasNoTracker)?
            .iter()
            .cloned()
            .collect();

        for (name, tracker, bucket) in trackers_to_update.into_iter() {
            let stop_signal = self.stop_signal.clone().unwrap();
            let semaphore = Arc::new(tokio::sync::Semaphore::new(10));
            let options = Arc::clone(&self.options.clone().unwrap());
            let tracker = Arc::clone(&tracker);
            let name = name.clone();

            tokio::spawn(Converter::spawn_conversion_task(
                name,
                tracker,
                bucket,
                options,
                stop_signal.clone(),
                semaphore.clone(),
            ));
        }

        // Reset internal state after spawning all tasks
        // self.trackers_to_update = None;
        self.state = State::Idle;
        self.stop_signal = None;

        Ok(())
    }

    async fn spawn_conversion_task(
        name: String,
        tracker: Arc<ProgressTracker>,
        bucket: Bucket,
        options: Arc<ConverterOptions>,
        stop_signal: broadcast::Sender<()>,
        semaphore: Arc<tokio::sync::Semaphore>,
    ) {
        let permit = semaphore.acquire_owned().await.unwrap();
        let mut ss = stop_signal.subscribe();

        tokio::select! {
            _ = ss.recv() => {
                // Stop signal received
                eprintln!("Conversion task for {} was interrupted.", name);
            }
            _ = Converter::convert(
                &options,
                &name,
                &bucket,
                tracker,
                stop_signal.clone()
            ) => {
                // Task completed or errored
                eprintln!("Conversion task for {} finished.", name);
            }
        }

        // Release the semaphore permit
        drop(permit);
    }

    // pub fn _start_conversion(self) -> Result<(Converter, tokio::sync::oneshot::Sender<()>)> {
    //     if matches!(self.state, State::Idle) {
    //         return Err(Error::ConverterHasNoTaskAvailable);
    //     }

    //     let options = self
    //         .options
    //         .expect("At this stage `self.options should be a Some`");
    //     // Now perform the parallel processing
    //     let (stop_tx, stop_rx) = tokio::sync::oneshot::channel();

    //     let stop_signal = Arc::new(tokio::sync::Mutex::new(stop_rx));

    //     let semaphore = Arc::new(tokio::sync::Semaphore::new(10));

    //     tokio::spawn(async move {
    //         let mut tasks = Arc::new(Vec::new());
    //         for (name, tracker, bucket) in self
    //             .trackers_to_update
    //             .expect("At this stage `trackers_to_update` should be a Some")
    //         {
    //             let tasks_clone = tasks.clone();
    //             let options = options.clone();
    //             let permit = semaphore.clone().acquire_owned().await.unwrap();
    //             let stop_signal = Arc::clone(&stop_signal);

    //             tasks_clone.push(tokio::spawn(async move {
    //                 // Create the output directory
    //                 let mut output = options.output_dir.clone();
    //                 output.push(&name);

    //                 // Create the output directory with correct permissions
    //                 if let Err(e) = create_directory_with_permissions(&output) {
    //                     eprintln!("Failed to create directory {:?}: {:?}", output, e);
    //                     return;
    //                 }

    //                 // Create the XML directory inside the output directory
    //                 let mut xml_dir = output.clone();
    //                 xml_dir.push("xml");
    //                 if let Err(e) = create_directory_with_permissions(&xml_dir) {
    //                     eprintln!("Failed to create directory {:?}: {:?}", xml_dir, e);
    //                     return;
    //                 }

    //                 // Copy the sorted XML files into the XML directory
    //                 if let Err(e) = copy_files(bucket.xml_files(), xml_dir, Some(tracker.clone())) {
    //                     eprintln!("Failed to copy files: {:?}", e);
    //                     return;
    //                 }

    //                 tracker.set_job_video();

    //                 // Start converting the video files
    //                 if let Err(e) = exec_batch_ffmpeg(
    //                     bucket.video_files(),
    //                     output,
    //                     options.ffmpeg_options,
    //                     Some(tracker.clone()),
    //                 ) {
    //                     eprintln!("FFmpeg execution failed: {:?}", e);
    //                 }

    //                 drop(permit);
    //             }));
    //         }
    //     });

    //     // Conversion process complete
    //     Ok((
    //         Converter {
    //             progress_trackers: self.progress_trackers,
    //             options: None,
    //             trackers_to_update: None,
    //             state: State::Idle,
    //             // stop_signal: self.stop_signal,
    //         },
    //         stop_tx,
    //     ))
    // }

    async fn convert(
        options: &ConverterOptions,
        name: &String,
        bucket: &Bucket,
        tracker: Arc<ProgressTracker>,
        stop_signal: broadcast::Sender<()>,
    ) -> Result<()> {
        // Create the output directory
        let mut output = options.output_dir.clone();
        output.push(name);

        if let Err(e) = create_directory_with_permissions(&output) {
            eprintln!("Failed to create directory {:?}: {:?}", output, e);
            return Err(e);
        }

        // Create the XML directory inside the output directory
        let mut xml_dir = output.clone();
        xml_dir.push("xml");
        if let Err(e) = create_directory_with_permissions(&xml_dir) {
            eprintln!("Failed to create directory {:?}: {:?}", xml_dir, e);
            return Err(e);
        }

        // Copy the sorted XML files into the XML directory
        if let Err(e) = copy_files(bucket.xml_files(), xml_dir, Some(tracker.clone())) {
            eprintln!("Failed to copy files: {:?}", e);
            return Err(e);
        }

        // Update the progress tracker to indicate the next job
        tracker.set_job_video();

        // Execute the FFmpeg batch processing with a stop signal
        exec_batch_ffmpeg(
            bucket.video_files(),
            output,
            options.ffmpeg_options,
            Some(tracker.clone()),
            stop_signal.clone(),
        )
        .await
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
