use crate::{Error, ProgressSystem, Result, Stage};
use std::{fs::File, io, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub(crate) async fn copy_files(
    files: Vec<PathBuf>,
    des: PathBuf,
    folder_name: String,
    tracker: Option<Arc<RwLock<ProgressSystem>>>,
) -> Result<()> {
    for file in files {
        let tracker = tracker.clone();
        copy_file(&file, des.clone())?;
        if let Some(tracker) = tracker {
            tracker
                .read()
                .await
                .update_progress(
                    folder_name.clone(),
                    Stage::Xml,
                    file.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .to_lowercase(),
                )
                .await
                .unwrap();
        }
    }
    Ok(())
}

pub(crate) fn copy_file(file: &PathBuf, des: PathBuf) -> Result<()> {
    // Open the source file
    let source_name = file.file_name().unwrap();
    let mut source_file = File::open(file)
        .map_err(|err| Error::CopyError(format!("Failed to open source file: {}", err)))?;

    // Create the destination file (fail if it already exists)
    let mut file_des = des.clone();
    file_des.push(source_name);
    let mut dest_file = File::create(&file_des)
        .map_err(|err| Error::CopyError(format!("Failed to create destination file: {}", err)))?;

    // Perform the file copy operation
    io::copy(&mut source_file, &mut dest_file)
        .map_err(|err| Error::CopyError(format!("Failed to copy file: {}", err)))?;

    Ok(())
}
