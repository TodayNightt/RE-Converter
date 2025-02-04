use std::{collections::HashMap, sync::Arc, time::Duration};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use lib_core::{Converter, ConverterOptions, ProgressInfo, ProgressMonitor, ProgressTracker};
// use no_deadlocks::RwLock;
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, RwLock},
};
use tokio_tungstenite::{accept_async, WebSocketStream};
use typeshare::typeshare;

use crate::State;

type WSMessage = tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Debug, Serialize, Deserialize)]
#[typeshare]
#[serde(rename_all = "camelCase", tag = "method", content = "data")]
pub enum Message {
    Hello,
    Convert(ConverterOptions),
    Acknowledge,
    CancelAcknowledge,
    Progress(Vec<ProgressInfo>),
    Cancel,
    Error(String),
    Finished,
}

pub async fn start_server(state: Arc<RwLock<State>>) {
    let addr = "127.0.0.1:8080".to_string(); // Use a non-privileged port for development

    println!("Starting WebSocket server @ ws://{}", addr);

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind WebSocket server");

    while let Ok((stream, _)) = listener.accept().await {
        let state_clone = Arc::clone(&state);
        tokio::spawn(accept_connection(stream, state_clone));
    }
}

async fn accept_connection(stream: TcpStream, state: Arc<RwLock<State>>) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during the WebSocket handshake");

    let (write, read) = ws_stream.split();

    tokio::spawn(async move {
        handle_message(write, read, state).await;
    });
}

async fn handle_message(
    mut write: SplitSink<WebSocketStream<TcpStream>, WSMessage>,
    mut read: SplitStream<WebSocketStream<TcpStream>>,
    state: Arc<RwLock<State>>,
) {
    // Channel for sending messages
    let (sender_tx, mut sender_rx) = tokio::sync::mpsc::channel::<WSMessage>(32);
    let sender_tx = Arc::new(sender_tx);

    // Task for sending messages to the client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = sender_rx.recv().await {
            if let Err(e) = write.send(msg).await {
                eprintln!("Error sending message: {}", e);
                break; // Exit the send loop on error
            }
        }
    });
    let progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>> = Arc::default();
    let converter = Arc::new(Mutex::new(Converter::new(progress_trackers.clone())));
    let monitor = Arc::new(Mutex::new(ProgressMonitor::new(progress_trackers)));

    let converter_stop_signal = Arc::new(Mutex::new(None));

    // Read and process incoming messages
    while let Some(message) = read.next().await {
        match message {
            Ok(WSMessage::Text(text)) => match serde_json::from_str::<Message>(&text) {
                Ok(msg) => match msg {
                    Message::Hello => {
                        let sender_tx_clone = Arc::clone(&sender_tx);
                        if let Err(e) = sender_tx_clone
                            .send(WSMessage::text("Hello world!!".to_string()))
                            .await
                        {
                            eprintln!("Failed to send Hello response: {}", e);
                        }
                    }
                    Message::Convert(options) => {
                        let options = Arc::new(options);
                        let sender_tx_clone = Arc::clone(&sender_tx);
                        let converter_clone = Arc::clone(&converter);
                        let stop_signal_clone = Arc::clone(&converter_stop_signal);

                        // Prepare the converter task and stop signal
                        {
                            let mut converter_guard = converter_clone.lock().await;
                            let mut stop_signal_guard = stop_signal_clone.lock().await;
                            let options_clone = Arc::clone(&options);
                            match converter_guard.prepare_task(options_clone).await {
                                Ok(stop_signal) => {
                                    *stop_signal_guard = Some(stop_signal);
                                }
                                Err(e) => {
                                    let _ = sender_tx_clone
                                        .send(WSMessage::text(
                                            serde_json::to_string(&Message::Error(e.to_string()))
                                                .unwrap(),
                                        ))
                                        .await;
                                    continue;
                                }
                            }
                        }

                        // Spawn the converter task
                        let monitor_clone = Arc::clone(&monitor);
                        let stc = Arc::clone(&sender_tx_clone);
                        let state_clone = Arc::clone(&state);
                        let options_clone = Arc::clone(&options);
                        let converter_task = tokio::spawn(async move {
                            let mut converter_guard = converter_clone.lock().await;
                            match converter_guard.start_conversion() {
                                Ok(_) => {
                                    let _ = stc
                                        .send(WSMessage::text(
                                            serde_json::to_string(&Message::Finished).unwrap(),
                                        ))
                                        .await;
                                    {
                                        let mut new_config =
                                            { state_clone.read().await.config.clone() };
                                        new_config
                                            .update_last_saved(options_clone.as_ref().clone());
                                        new_config
                                    };
                                }
                                Err(e) => {
                                    let _ = stc
                                        .send(WSMessage::text(
                                            serde_json::to_string(&Message::Error(e.to_string()))
                                                .unwrap(),
                                        ))
                                        .await;
                                    converter_guard.reset();
                                    monitor_clone.lock().await.stop().await;
                                }
                            }
                        });

                        // Spawn a progress-tracking task
                        let stc = Arc::clone(&sender_tx_clone);
                        let monitor_clone = Arc::clone(&monitor);
                        let progress_task = tokio::spawn(async move {
                            let mut progress_guard = monitor_clone.lock().await;
                            progress_guard.start(
                                move |infos| {
                                    let stcc = Arc::clone(&stc);
                                    async move {
                                        if let Err(e) = stcc
                                            .send(WSMessage::text(
                                                serde_json::to_string(&Message::Progress(infos))
                                                    .unwrap(),
                                            ))
                                            .await
                                        {
                                            eprintln!("Failed to send progress update: {}", e);
                                        }
                                    }
                                },
                                Duration::from_secs(1),
                            )
                        });

                        if let Err(e) = sender_tx_clone
                            .send(WSMessage::text(
                                serde_json::to_string(&Message::Acknowledge).unwrap(),
                            ))
                            .await
                        {
                            eprintln!("Failed to send acknowledge message: {}", e);
                        }

                        // Wait for tasks to complete
                        let (converter_result, _) = tokio::join!(converter_task, progress_task);

                        if let Err(e) = converter_result {
                            eprintln!("Converter task error: {:?}", e);
                        }

                        let sender_tx_clone = Arc::clone(&sender_tx);
                        sender_tx_clone
                            .send(WSMessage::text(
                                serde_json::to_string(&Message::Finished).unwrap(),
                            ))
                            .await
                            .unwrap();
                    }
                    Message::Cancel => {
                        // Send the cancellation signal
                        {
                            let stop_signal_guard = converter_stop_signal.lock().await;
                            if let Some(stop_signal) = stop_signal_guard.as_ref() {
                                if let Err(e) = stop_signal.send(()) {
                                    eprintln!("Failed to send stop signal: {:?}", e);
                                } else {
                                    println!("Conversion task cancellation requested.");
                                }
                            } else {
                                println!("No active conversion task to cancel.");
                            }

                            let mut progress_guard = monitor.lock().await;
                            progress_guard.stop().await;
                        }

                        // Reset the converter state
                        {
                            let mut converter_guard = converter.lock().await;
                            converter_guard.reset();
                        }

                        // Optionally notify the client about cancellation
                        let sender_tx_clone = Arc::clone(&sender_tx);
                        if let Err(e) = sender_tx_clone
                            .send(WSMessage::text(
                                serde_json::to_string(&Message::CancelAcknowledge).unwrap(),
                            ))
                            .await
                        {
                            eprintln!("Failed to send cancel acknowledgment: {:?}", e);
                        }
                    }

                    _ => (),
                },
                Err(e) => {
                    let sender_tx_clone = Arc::clone(&sender_tx);

                    if let Err(err) = sender_tx_clone
                        .send(WSMessage::text(
                            serde_json::to_string(&Message::Error(e.to_string())).unwrap(),
                        ))
                        .await
                    {
                        eprintln!("Failed to send Error message: {}", err);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading WebSocket message: {}", e);
                break; // Exit the loop on error
            }
            _ => (),
        }
    }

    // Cleanup: drop the sender and wait for the send task to complete
    drop(sender_tx);
    if let Err(e) = send_task.await {
        eprintln!("Error in send task: {}", e);
    }
}
