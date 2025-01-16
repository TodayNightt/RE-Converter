// use std::{
//     collections::HashMap,
//     sync::{mpsc::Receiver, Arc},
// };

// use lib_core::{Converter, ConverterOptions, ProgressInfo, ProgressMonitor, ProgressTracker};
// use no_deadlocks::{Mutex, RwLock};
// use serde::{Deserialize, Serialize};
// use tauri::{ipc::Channel, AppHandle, Manager};
// use tokio::{
//     runtime::Runtime,
//     sync::{mpsc, oneshot},
// };
// use typeshare::typeshare;

// #[typeshare]
// #[derive(Serialize, Clone, Deserialize)]
// struct ProgressPayload {
//     progress_list: Vec<ProgressInfo>,
// }

// #[typeshare]
// #[derive(Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase", tag = "event", content = "data")]
// pub enum ConvertEvent {
//     Established,

//     #[serde(rename_all = "camelCase")]
//     Progress {
//         progress: Vec<ProgressInfo>,
//     },
//     Canceled,
//     Finished,
// }

// #[tauri::command]
// pub async fn convert(
//     _app: AppHandle,
//     options: ConverterOptions,
//     btf: Channel<ConvertEvent>,
//     ftb: Channel<ConvertEvent>,
// ) {
//     // Create a channel to receive messages from the frontend
//     let (tx, rx) = oneshot::channel();

//     println!("{:?}", options);
//     let progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>> = Arc::default();
//     let progress_monitor = Arc::new(Mutex::new(ProgressMonitor::new(progress_trackers.clone())));
//     let pm_clone = progress_monitor.clone();
//     let pt_clone = progress_trackers.clone();

//     // Spawn the conversion process in a thread
//     tokio::spawn(async move {
//         tokio::pin!(rx);
//         if let Err(err) = Converter::convert(options, pt_clone) {
//             eprintln!("Conversion error: {:?}", err);
//             progress_monitor.lock().unwrap().stop().await;
//         }
//     });

//     // Handle progress updates
//     progress(btf, pm_clone).await;

//     ftb.
// }

// async fn progress(on_event: Channel<ConvertEvent>, progress_monitor: Arc<Mutex<ProgressMonitor>>) {
//     progress_monitor.lock().unwrap().start(
//         move |info| {
//             if let Err(err) = on_event.send(ConvertEvent::Progress { progress: info }) {
//                 eprintln!("Error sending progress: {:?}", err);
//             }
//         },
//         std::time::Duration::from_millis(100), // Interval for updates
//     );
// }
