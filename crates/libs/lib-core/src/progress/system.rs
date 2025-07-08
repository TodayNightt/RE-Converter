use crate::{
    progress::{JobInfo, Message},
    Progress, ProgressMonitor, Stage,
};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::{
    sync::mpsc::{Receiver, Sender},
    time::interval,
};

#[derive(Debug)]
pub struct ProgressSystem {
    message_tx: Sender<Message>,
    progress_rx: Receiver<Vec<Progress>>,
    prog_mon_handle: JoinHandle<()>,
}

impl Drop for ProgressSystem {
    fn drop(&mut self) {
        self.prog_mon_handle.abort();
    }
}

impl ProgressSystem {
    pub fn new(update_interval: u64) -> Self {
        let (message_tx, message_rx) = tokio::sync::mpsc::channel(100);
        let (progress_tx, progress_rx) = tokio::sync::mpsc::channel(10);
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

    pub async fn create_tracker(&self, job_info: JobInfo) -> Result<(), String> {
        self.message_tx
            .send(Message::Create { job_info })
            .await
            .map_err(|e| format!("Failed to send create message: {}", e))
    }

    pub async fn update_progress(
        &self,
        folder_name: String,
        stage: Stage,
        working_file: String,
    ) -> Result<(), String> {
        self.message_tx
            .send(Message::Update {
                folder_name,
                working_file,
                action: stage,
            })
            .await
            .map_err(|e| format!("Failed to send update message: {}", e))
    }

    pub async fn done(&self, folder_name: String) -> Result<(), String> {
        self.message_tx
            .send(Message::Done { folder_name })
            .await
            .map_err(|e| format!("Failed to send done message: {}", e))
    }

    pub async fn get_progress(&mut self) -> Option<Vec<Progress>> {
        self.progress_rx.recv().await
    }
}
