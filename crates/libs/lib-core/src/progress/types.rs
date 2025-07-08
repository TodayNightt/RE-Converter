use std::sync::atomic::AtomicBool;

use crate::progress::tracker::Stage;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Serialize, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobInfo {
    folder_name: String,
    total_video: u32,
    total_xml: u32,
}
impl JobInfo {
    pub fn new(folder_name: String, total_video: usize, total_xml: usize) -> Self {
        Self {
            folder_name,
            total_video: total_video as u32,
            total_xml: total_xml as u32,
        }
    }
    pub fn folder_name(&self) -> String {
        self.folder_name.clone()
    }

    pub fn total_video(&self) -> u32 {
        self.total_video
    }

    pub fn total_xml(&self) -> u32 {
        self.total_xml
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Create {
        job_info: JobInfo,
    },
    Update {
        folder_name: String,
        working_file: String,
        action: Stage,
    },
    Done {
        folder_name: String,
    },
}

#[derive(Clone, Debug)]
pub struct Progress {
    folder: String,
    file: String,
    count: u8,
    stage: Stage,
    error_count: u8,
    total: u8,
    done: bool,
}

impl Progress {
    pub fn new(
        folder: String,
        file: String,
        count: u8,
        stage: Stage,
        error_count: u8,
        total: u8,
        done: bool,
    ) -> Self {
        Self {
            folder,
            file,
            count,
            stage,
            error_count,
            total,
            done,
        }
    }

    pub fn folder(&self) -> String {
        self.folder.clone()
    }

    pub fn file(&self) -> String {
        self.file.clone()
    }

    pub fn total(&self) -> u8 {
        self.total
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn error_count(&self) -> u8 {
        self.error_count
    }

    pub fn stage(&self) -> &str {
        match self.stage {
            Stage::Xml => "Xml",
            Stage::Video => "Video",
        }
    }

    pub fn done(&self) -> bool {
        self.done
    }
}
