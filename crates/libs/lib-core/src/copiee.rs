use crate::{Error, ProgressSystem, Result, Stage};
use std::path::Path;
use std::{fs::File, io, path::PathBuf, sync::Arc};
use tokio::io::BufReader;
use tokio::sync::RwLock;

pub(crate) async fn copy_files(
    files: Arc<[PathBuf]>,
    des: &Path,
    folder_name: Arc<str>,
    tracker: Option<Arc<RwLock<ProgressSystem>>>,
) -> Result<()> {
    tracing::info!("Copying files [{}]", folder_name);

    // let result: Vec<Result<()>> = stream::iter(files)
    //     .map(|file| {
    //         let tracker = tracker.clone();
    //
    //         async move {
    //             tracing::debug!("Starting copy for file: {:?}", file);
    //
    //             // Copy the file
    //             copy_file(&file, des).await?;
    //
    //             // Update progress tracker
    //             if let Some(tracker) = tracker {
    //                 tracing::info!("Updating tracker for file : {:?} [{}]", file, folder_name);
    //
    //                 tracker
    //                     .read()
    //                     .await
    //                     .update_progress(
    //                         folder_name,
    //                         Stage::Xml,
    //                         &file.file_name().unwrap().to_str().unwrap().to_lowercase(),
    //                     )
    //                     .await?;
    //             }
    //
    //             Ok(())
    //         }
    //     })
    //     .buffer_unordered(10)
    //     .collect()
    //     .await;
    //
    // for r in result {
    //     r?
    // }

    for file in files.iter() {
        let tracker = tracker.clone();

        copy_file(&file, des)?;
        if let Some(tracker) = tracker {
            tracing::info!("Updating tracker for file : {:?} [{}]", file, folder_name);

            tracker
                .read()
                .await
                .update_progress(
                    folder_name.clone(),
                    Stage::Xml,
                    &file.file_name().unwrap().to_str().unwrap().to_lowercase(),
                )
                .await?;
        }
    }
    Ok(())
}

pub(crate) fn copy_file(file: &Path, des: &Path) -> Result<()> {
    // Open the source file
    let source_name = file.file_name().unwrap();
    let mut source_file = File::open(file)
        .map_err(|err| Error::CopyError(format!("Failed to open source file: {}", err)))?;

    // Create the destination file (fail if it already exists)
    let mut file_des = des.to_path_buf();
    file_des.push(source_name);

    if file_des.exists() {
        return Ok(());
    }
    let mut dest_file = File::create(&file_des)
        .map_err(|err| Error::CopyError(format!("Failed to create destination file: {}", err)))?;

    // Perform the file copy operation
    io::copy(&mut source_file, &mut dest_file)
        .map_err(|err| Error::CopyError(format!("Failed to copy file: {}", err)))?;

    Ok(())
}
