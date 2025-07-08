use crate::progress::{
    tracker::{ProgressTracker, Stage},
    Message, Progress,
};
use std::collections::HashMap;
use tokio::{
    select,
    sync::{
        mpsc::{Receiver, Sender},
        RwLock,
    },
    time::Interval,
};

pub struct ProgressMonitor {
    progress_trackers: RwLock<HashMap<String, ProgressTracker>>,
    message_rx: Receiver<Message>,
    progress_tx: Sender<Vec<Progress>>,
    update_interval: Interval,
}

impl ProgressMonitor {
    pub fn new(
        message_rx: Receiver<Message>,
        progress_tx: Sender<Vec<Progress>>,
        update_interval: Interval,
    ) -> Self {
        Self {
            progress_trackers: RwLock::new(HashMap::new()),
            message_rx,
            progress_tx,
            update_interval,
        }
    }

    pub async fn start(&mut self) {
        println!("Starting progress monitor");
        loop {
            if self.message_rx.is_closed() {
                break;
            }

            select! {
                data = self.message_rx.recv()=>{
                    if let Some(data) = data{

                        match data {
                    Message::Create{job_info} => {
                        let key = job_info.folder_name();
                        let tracker = ProgressTracker::new(job_info);
                        // Create the tracker object
                        {
                            self.progress_trackers
                                .write()
                                .await
                                .entry(key)
                                .or_insert(tracker);
                        }
                    }

                    Message::Update {
                        folder_name,
                        action,
                                working_file,
                    } => {
                        self.progress_trackers
                            .write()
                            .await
                            .entry(folder_name)
                            .and_modify(|tracker| match action {
                                Stage::Xml => tracker.update_xml(working_file).unwrap(),
                                Stage::Video => tracker.update_video(working_file).unwrap(),
                            });
                    }
                            Message::Done {folder_name} =>{
                                self.progress_trackers.write().await.entry(folder_name).and_modify(|tracker| tracker.set_done());
                            }
                    }
                }
                    }

                _ = self.update_interval.tick() =>{
                    let progress = {
                        let progress_data = self.progress_trackers.read().await;
                         progress_data.values().map(|tracker| tracker.progress()).collect()
                    };
                        let _ = self.progress_tx.send(progress).await;

                }
            }
        }
    }
}
