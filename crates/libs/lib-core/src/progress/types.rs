use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use crate::progress::tracker::Stage;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone)]
// #[serde(rename_all = "camelCase")]
pub struct JobInfo {
    folder_name: Arc<str>,
    total_video: u32,
    total_xml: u32,
}
impl JobInfo {
    pub fn new(folder_name: Arc<str>, total_video: usize, total_xml: usize) -> Self {
        Self {
            folder_name,
            total_video: total_video as u32,
            total_xml: total_xml as u32,
        }
    }
    pub fn folder_name(&self) -> Arc<str> {
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
        folder_name: Arc<str>,
        working_file: Arc<str>,
        action: Stage,
    },
    Done {
        folder_name: Arc<str>,
    },
}

#[derive(Clone, Debug)]
pub struct Progress {
    folder: Arc<str>,
    file: Arc<str>,
    count: u8,
    stage: Stage,
    error_count: u8,
    total: u8,
    done: bool,
}

impl Progress {
    pub fn new(
        folder: Arc<str>,
        file: Arc<str>,
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

    pub fn folder(&self) -> Arc<str> {
        self.folder.clone()
    }

    pub fn file(&self) -> Arc<str> {
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
