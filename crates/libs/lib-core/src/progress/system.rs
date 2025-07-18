use crate::{
    progress::{Error, JobInfo, Message, Result},
    Progress, ProgressMonitor, Stage,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::interval,
};

#[derive(Debug)]
pub struct ProgressSystem {
    message_tx: Sender<Message>,
    progress_rx: Receiver<Arc<[Progress]>>,
    prog_mon_handle: JoinHandle<()>,
}

impl Drop for ProgressSystem {
    fn drop(&mut self) {
        self.prog_mon_handle.abort();
    }
}

impl ProgressSystem {
    pub fn new(update_interval: u64) -> Self {
        let (message_tx, message_rx) = tokio::sync::mpsc::channel(600);
        let (progress_tx, progress_rx) = tokio::sync::mpsc::channel(30);
        let mut progress_monitor = ProgressMonitor::new(
            message_rx,
            progress_tx,
            interval(Duration::from_millis(update_interval)),
        );

        let prog_mon_handle = tokio::spawn(async move {
            progress_monitor.start().await;
        });

        Self {
            message_tx,
            progress_rx,
            prog_mon_handle,
        }
    }

    pub async fn create_tracker(&self, job_info: &JobInfo) -> Result<()> {
        self.message_tx
            .send(Message::Create {
                job_info: job_info.to_owned(),
            })
            .await
            .map_err(|_| Error::CreateSignalFailed(job_info.folder_name()))
    }

    pub async fn update_progress(
        &self,
        folder_name: Arc<str>,
        stage: Stage,
        working_file: &str,
    ) -> Result<()> {
        self.message_tx
            .send(Message::Update {
                folder_name: folder_name.clone(),
                working_file: Arc::from(working_file),
                action: stage,
            })
            .await
            .map_err(|_| Error::UpdateSignalFailed(working_file.to_string(), folder_name))
    }

    pub async fn done(&self, folder_name: Arc<str>) -> Result<()> {
        self.message_tx
            .send(Message::Done {
                folder_name: folder_name.clone(),
            })
            .await
            .map_err(|_| Error::DoneSignalFailed(folder_name))
    }

    pub async fn get_progress(&mut self) -> Option<Arc<[Progress]>> {
        self.progress_rx.recv().await
    }
}
