use std::sync::RwLock;

// use no_deadlocks::prelude::RwLock;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[derive(Debug, Clone, Copy)]
enum CurrentJob {
    Xml,
    Video,
}

#[derive(Debug)]
pub struct ProgressTracker {
    folder_name: String,
    file_name: RwLock<String>,
    job: RwLock<CurrentJob>,
    current_xml: RwLock<u32>,
    total_xml: u32,
    current_video: RwLock<u32>,
    total_video: u32,
}

#[typeshare]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressInfo {
    folder_name: String,
    // Note : This can change into a enum
    status: String,
    file_name: String,
    current_progress: u32,
    total_progress: u32,
}

impl ProgressTracker {
    pub fn new(total_xml: u32, total_video: u32, folder_name: impl Into<String>) -> Self {
        ProgressTracker {
            folder_name: folder_name.into(),
            file_name: RwLock::default(),
            job: RwLock::new(CurrentJob::Xml),
            current_xml: RwLock::new(0),
            total_xml,
            current_video: RwLock::new(0),
            total_video,
        }
    }

    pub fn progress(&self) -> ProgressInfo {
        match *self.job.read().unwrap() {
            CurrentJob::Xml => ProgressInfo {
                folder_name: self.folder_name.clone(),
                status: "Copying XML files to destination".to_string(),
                file_name: self.file_name.read().unwrap().to_string(),
                current_progress: *self.current_xml.read().unwrap(),
                total_progress: self.total_xml,
            },
            CurrentJob::Video => ProgressInfo {
                folder_name: self.folder_name.clone(),
                status: "Transcoding Video files and copy it to destination".to_string(),
                file_name: self.file_name.read().unwrap().to_string(),
                current_progress: *self.current_video.read().unwrap(),
                total_progress: self.total_video,
            },
        }
    }

    pub fn set_job_xml(&self) {
        *self.job.write().unwrap() = CurrentJob::Xml;
    }

    pub fn set_job_video(&self) {
        *self.job.write().unwrap() = CurrentJob::Video;
    }

    pub fn complete_one(&self, next: impl Into<String>) {
        *self.file_name.write().unwrap() = next.into();
        match *self.job.read().unwrap() {
            CurrentJob::Xml => *self.current_xml.write().unwrap() += 1,
            CurrentJob::Video => *self.current_video.write().unwrap() += 1,
        }
    }

    pub fn check_completed(&self) -> bool {
        let j = *self.job.read().unwrap();
        match j {
            CurrentJob::Xml => {
                if *self.current_xml.read().unwrap() == self.total_xml {
                    *self.job.write().unwrap() = CurrentJob::Video;
                }
            }
            CurrentJob::Video => {
                if *self.current_video.read().unwrap() == self.total_video {
                    return true;
                }
            }
        }
        false
    }
}
