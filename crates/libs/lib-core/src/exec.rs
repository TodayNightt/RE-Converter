const CREATE_NO_WINDOW: u32 = 0x08000000;
use std::{path::PathBuf, process::Stdio, sync::Arc};

use lib_utils::file::FileExt;
use tokio::{process::Child, sync::RwLock};

use crate::{converter::FfmpegOptions, Error, ProgressSystem, Result, Stage};
use tokio::{process::Command, select, sync::watch::Receiver, task::JoinSet};

pub async fn exec_batch_ffmpeg(
    files: Arc<[FileExt]>,
    des: PathBuf,
    flag: FfmpegOptions,
    stop_signal: Receiver<bool>, // Add the stop signal
    ffmpeg_executable: Option<&'static PathBuf>,
    progress_system: Option<Arc<RwLock<ProgressSystem>>>,
    folder_name: Arc<str>,
) -> Result<()> {
    let mut join_set = JoinSet::new();

    let semaphore = Arc::new(tokio::sync::Semaphore::new(2));

    let files = files.clone();

    for file in files.iter() {
        let semaphore = semaphore.clone();
        let des = des.clone();
        let mut stop_signal = stop_signal.clone();
        let file_name = file
            .file_name()
            .to_str()
            .unwrap()
            .to_string()
            .to_lowercase();
        let folder_name = folder_name.to_owned();
        let progress_system = progress_system.clone();

        let file = file.clone();

        join_set.spawn(async move {

            let permit = semaphore.acquire_owned().await.unwrap();
            let mut child = exec_ffmpeg(file, des, flag, ffmpeg_executable)
                .await
                .unwrap();

            let mut stderr = child.stderr.take().unwrap();

            select! {
                _ = stop_signal.changed() =>{
                    let should_stop = *stop_signal.wait_for(|val|*val).await.unwrap();
            
                    if should_stop{
                       child.kill().await.unwrap();
                    }

                    tracing::info!("Killing execution for file : {}",file_name);
                }
            
                status = child.wait() =>{
                    if let Ok(output) =status {
                        if !output.success() {
                            let mut err_output = String::new();
                            use tokio::io::AsyncReadExt;
                            stderr.read_to_string(&mut err_output).await.unwrap();
                            tracing::error!("File : {:?}[{}]\nstderr : {}\n",file_name,folder_name,err_output);

                            return Err(Error::FfmpegError(err_output));
                        }
            
                        if let Some(tracker) = progress_system{
                            tracker.write().await.update_progress(folder_name, Stage::Video,&file_name).await?;
                        }
            
                    }
                }
            
            
            }

            drop(permit);

            Ok(())
        });
    }
    let mut ss = stop_signal.clone();

    loop {
        select! {
           _ = ss.changed() => {
                if *ss.borrow() {
                    // First shutdown to prevent new tasks
                    join_set.detach_all();
                    break;
                }
            }

            result = join_set.join_next()=>{
                if result.is_none(){
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn exec_ffmpeg(
    source: FileExt,
    des: PathBuf,
    flag: FfmpegOptions,
    ffmpeg_executable: Option<&'static PathBuf>,
) -> Result<Child> {
    let source_name = &source.file_name();
    let mut file_des = des.clone();
    file_des.push(source_name.as_ref());

    let args = flag.build(
        source.path_with_extension(),
        file_des.with_extension(flag.output_extension.to_string()),
    );

    let command;

    if let Some(ffmpeg_executable) = ffmpeg_executable {
        command = Some(Command::new(ffmpeg_executable));
    } else {
        command = Some(Command::new("ffmpeg"));
    }

    command
        .unwrap()
        .args(args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|err| Error::FfmpegError(format!("Failed to execute FFmpeg: {:?}", err)))
}
