use std::{collections::HashMap, future::Future, sync::Arc, time::Duration};

use tokio::{
    sync::{oneshot, RwLock},
    task::JoinHandle,
    time::sleep,
};

use super::{tracker::ProgressTracker, ProgressInfo};

pub struct ProgressMonitor {
    progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>>,
    task_handle: Option<JoinHandle<()>>,
    stop_signal: Option<oneshot::Sender<()>>,
}

impl ProgressMonitor {
    pub fn new(progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>>) -> Self {
        Self {
            progress_trackers,
            task_handle: None,
            stop_signal: None,
        }
    }

    pub fn start<T, F>(&mut self, func_ptr: F, interval: Duration)
    where
        F: Fn(Vec<ProgressInfo>) -> T + Send + Sync + 'static,
        T: Future<Output = ()> + Send + 'static,
    {
        if self.is_running() {
            println!("Task is already running!");
            return;
        }

        println!("Starting progress monitor");

        let (stop_tx, stop_rx) = oneshot::channel();
        let trackers = Arc::clone(&self.progress_trackers);

        let handle = tokio::spawn(async move {
            tokio::pin!(stop_rx);

            println!("Hello from future");
            loop {
                let trackers: Vec<_> = { trackers.read().await.values().cloned().collect() };

                println!("Hello from future loop");
                if stop_rx.try_recv().is_ok() {
                    println!("Stopping background task.");
                    break;
                }

                let mut infos = Vec::new();
                // if let Ok(tracker_guard) = trackers.read() {
                trackers.iter().for_each(|pt| {
                    println!("Accquired read lock");
                    infos.push(pt.progress());
                });

                // } else {
                //     eprintln!("Failed to acquire read lock on progress trackers.");
                //     continue;
                // }

                func_ptr(infos).await;
                sleep(interval).await;
            }
        });

        self.task_handle = Some(handle);
        self.stop_signal = Some(stop_tx);
    }

    pub async fn stop(&mut self) {
        if let Some(stop_tx) = self.stop_signal.take() {
            let _ = stop_tx.send(());
        }
        if let Some(handle) = self.task_handle.take() {
            let _ = handle.await;
        }
    }

    pub fn is_running(&self) -> bool {
        self.task_handle.is_some()
    }
}
