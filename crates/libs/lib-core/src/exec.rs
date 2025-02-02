const CREATE_NO_WINDOW: u32 = 0x08000000;
use std::{path::PathBuf, process::Stdio, sync::Arc};

use lib_utils::file::FileExt;

use tokio::{process::Command, sync::broadcast};

use crate::{converter::FfmpegOptions, ProgressTracker};

use super::{Error, Result};

pub async fn exec_batch_ffmpeg(
    files: Vec<FileExt>,
    des: PathBuf,
    flag: FfmpegOptions,
    progress_tracker: Option<Arc<ProgressTracker>>,
    stop_signal: broadcast::Sender<()>, // Add the stop signal
) -> Result<()> {
    let mut stop_receiver = stop_signal.subscribe(); // Create a subscriber for the stop signal

    for file in files {
        tokio::select! {
            // Stop signal is received
            _ = stop_receiver.recv() => {
                eprintln!("Stop signal received, halting batch execution.");
                break;
            }
            // Proceed with FFmpeg execution
            _ = async {
                if let Some(progress_tracker) = &progress_tracker {
                    progress_tracker.complete_one(file.file_name().to_str().unwrap());
                }
                if let Err(e) = exec_ffmpeg(file.clone(), des.clone(), flag).await {
                    eprintln!("Error processing file {:?}: {:?}", file, e);
                }
            } => {}
        }
    }

    Ok(())
}

async fn exec_ffmpeg(source: FileExt, des: PathBuf, flag: FfmpegOptions) -> Result<()> {
    let source_name = &source.file_name();
    let mut file_des = des.clone();
    file_des.push(source_name);

    let args = flag.build(
        source.path_with_extension(),
        file_des.with_extension(flag.output_extension.to_string()),
    );

    let output = Command::new("ffmpeg")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .await
        .map_err(|err| Error::FfmpegError(format!("Failed to execute FFmpeg: {:?}", err)))?;

    if !output.status.success() {
        return Err(Error::FfmpegError(format!(
            "{} -- {} (error_code: {})",
            source_name.to_string_lossy(),
            String::from_utf8_lossy(&output.stderr),
            output.status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}
