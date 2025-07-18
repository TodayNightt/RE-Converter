use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use crate::progress::{JobInfo, Progress};

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    Xml,
    Video,
}

#[derive(Debug)]
pub struct ProgressTracker {
    job_info: JobInfo,
    current_file: Arc<str>,
    current_xml: AtomicU32,
    current_video: AtomicU32,
    status: JobStatus,
    stage: Stage,
    errored: Vec<ErrorInfo>,
}

#[derive(Debug)]
struct ErrorInfo {
    _file: String,
    _cause: String,
}

#[derive(Debug)]
enum JobStatus {
    Starting,
    Pending,
    Done,
}

impl ProgressTracker {
    pub fn new(job_info: JobInfo) -> Self {
        let capacity = job_info.total_xml() + job_info.total_video();
        ProgressTracker {
            job_info,
            status: JobStatus::Pending,
            errored: Vec::with_capacity(capacity as usize),
            current_xml: AtomicU32::new(0),
            current_video: AtomicU32::new(0),
            stage: Stage::Xml,
            current_file: Arc::default(),
        }
    }
    pub fn progress(&self) -> Progress {
        let (count, total) = match self.stage {
            Stage::Xml => (
                self.current_xml.load(Ordering::SeqCst),
                self.job_info.total_xml(),
            ),
            Stage::Video => (
                self.current_video.load(Ordering::SeqCst),
                self.job_info.total_video(),
            ),
        };
        Progress::new(
            self.job_info.folder_name(),
            self.current_file.to_owned(),
            count as u8,
            self.stage.to_owned(),
            self.errored.len() as u8,
            total as u8,
            matches!(self.status, JobStatus::Done),
        )
    }

    fn update(&mut self, update_request: Stage, working_file: Arc<str>) -> Result<(), String> {
        if matches!(self.status, JobStatus::Pending) {
            self.status = JobStatus::Starting;
        }
        if self
            .current_video
            .load(Ordering::SeqCst)
            .eq(&self.job_info.total_video())
            && self
                .current_xml
                .load(Ordering::SeqCst)
                .eq(&self.job_info.total_xml())
        {
            self.status = JobStatus::Done;
        }

        match update_request {
            Stage::Xml => {
                if self
                    .current_xml
                    .load(Ordering::SeqCst)
                    .eq(&self.job_info.total_xml())
                {
                    return Err("XML COPYING HAD BEEN DONE".to_string());
                }
                self.current_xml.fetch_add(1, Ordering::Relaxed);
                if self
                    .current_xml
                    .load(Ordering::SeqCst)
                    .eq(&self.job_info.total_xml())
                {
                    self.stage = Stage::Video;
                }
            }
            Stage::Video => {
                if self
                    .current_video
                    .load(Ordering::Relaxed)
                    .eq(&self.job_info.total_video())
                {
                    return Err("The things had been Done".to_string());
                }
                self.current_video.fetch_add(1, Ordering::SeqCst);
            }
        }

        self.current_file = working_file;

        Ok(())
    }

    pub fn update_xml(&mut self, working_file: Arc<str>) -> Result<(), String> {
        self.update(Stage::Xml, working_file)
    }
    pub fn update_video(&mut self, working_file: Arc<str>) -> Result<(), String> {
        self.update(Stage::Video, working_file)
    }

    pub fn set_done(&mut self) {
        self.status = JobStatus::Done;
    }
}
