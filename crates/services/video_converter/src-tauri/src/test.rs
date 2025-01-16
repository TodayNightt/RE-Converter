async fn handle_message(
    mut write: SplitSink<WebSocketStream<TcpStream>, WSMessage>,
    mut read: SplitStream<WebSocketStream<TcpStream>>,
) {
    // Channel for sending messages
    let (tx, mut rx) = tokio::sync::mpsc::channel::<WSMessage>(32);

    // Task for sending messages to the client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = write.send(msg).await {
                eprintln!("Error sending message: {}", e);
                break; // Exit the send loop on error
            }
        }
    });

    let progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>> = Arc::default();
    let mut converter: Converter<_> = Converter::new(progress_trackers.clone());
    let monitor = Arc::new(Mutex::new(ProgressMonitor::new(progress_trackers.clone())));

    let mut conversion_task = None;
    let mut monitor_task = None;

    // Read and process incoming messages
    while let Some(message) = read.next().await {
        match message {
            Ok(WSMessage::Text(text)) => match serde_json::from_str::<Message>(&text) {
                Ok(msg) => match msg {
                    Message::Hello => {
                        if let Err(e) = tx.send(WSMessage::Text("Hello world!!".to_string())).await
                        {
                            eprintln!("Failed to send Hello response: {}", e);
                        }
                    }
                    Message::Convert(options) => {
                        // Stop any existing tasks before starting a new conversion
                        if let Some(task) = conversion_task.take() {
                            task.abort();
                        }
                        if let Some(task) = monitor_task.take() {
                            task.abort();
                        }

                        match converter.prepare_task(options) {
                            Ok(task_converter) => {
                                converter = task_converter;

                                let tx_clone = tx.clone();
                                let monitor_clone = monitor.clone();

                                // Start conversion task
                                conversion_task = Some(tokio::spawn(async move {
                                    if let Err(e) = converter.convert() {
                                        eprintln!("Conversion failed: {}", e);
                                    } else {
                                        // Notify client when conversion is finished
                                        if let Err(e) = tx_clone
                                            .send(WSMessage::Text(
                                                serde_json::to_string(&Message::Finished).unwrap(),
                                            ))
                                            .await
                                        {
                                            eprintln!("Failed to send Finished message: {}", e);
                                        }
                                    }
                                }));

                                // Start progress monitor task
                                monitor_task = Some(tokio::spawn(async move {
                                    monitor_clone.lock().unwrap().start(
                                        move |infos| {
                                            let tx = tx_clone.clone();
                                            tokio::spawn(async move {
                                                if let Err(e) = tx
                                                    .send(WSMessage::Text(
                                                        serde_json::to_string(&Message::Progress(
                                                            infos,
                                                        ))
                                                        .unwrap(),
                                                    ))
                                                    .await
                                                {
                                                    eprintln!(
                                                        "Failed to send progress update: {}",
                                                        e
                                                    );
                                                }
                                            });
                                        },
                                        Duration::from_secs(1),
                                    );
                                }));

                                if let Err(e) = tx
                                    .send(WSMessage::Text(
                                        serde_json::to_string(&Message::Acknowledge).unwrap(),
                                    ))
                                    .await
                                {
                                    eprintln!("Failed to send Acknowledge: {}", e);
                                }
                            }
                            Err(e) => {
                                if let Err(err) = tx
                                    .send(WSMessage::Text(
                                        serde_json::to_string(&Message::Error(e.to_string()))
                                            .unwrap(),
                                    ))
                                    .await
                                {
                                    eprintln!("Failed to send Error message: {}", err);
                                }
                            }
                        }
                    }
                    Message::Cancel => {
                        if let Some(task) = conversion_task.take() {
                            task.abort();
                        }
                        if let Some(task) = monitor_task.take() {
                            task.abort();
                        }
                        if let Err(e) = tx
                            .send(WSMessage::Text(
                                serde_json::to_string(&Message::Acknowledge).unwrap(),
                            ))
                            .await
                        {
                            eprintln!("Failed to send Cancel Acknowledge: {}", e);
                        }
                    }
                    _ => (),
                },
                Err(e) => {
                    if let Err(err) = tx
                        .send(WSMessage::Text(
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

    // Cleanup: Stop any remaining tasks
    if let Some(task) = conversion_task.take() {
        task.abort();
    }
    if let Some(task) = monitor_task.take() {
        task.abort();
    }

    // Drop the sender and wait for the send task to complete
    drop(tx);
    if let Err(e) = send_task.await {
        eprintln!("Error in send task: {}", e);
    }
}
