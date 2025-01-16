use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use no_deadlocks::prelude::*;

use lib_core::{ProgressMonitor, ProgressTracker};
use rayon::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let progress_trackers: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>> = Arc::default();

    let mut progress_monitor = ProgressMonitor::new(progress_trackers.clone());

    progress_monitor.start(|infos| {
        print!("\x1B[2J\x1B[1;1H");
        println!("{:?}", serde_json::to_string_pretty(&infos).unwrap());
    });

    for i in 1..=25 {
        let mut pt = progress_trackers.write().unwrap();
        pt.entry(i.to_string())
            .or_insert_with(|| Arc::new(ProgressTracker::new(39, 30, i.to_string()  )));
    }

    make_stuff(progress_trackers.clone()).await;
    progress_monitor.stop().await;
    Ok(())
}

async fn make_stuff(pts: Arc<RwLock<HashMap<String, Arc<ProgressTracker>>>>) {
    let trackers = pts.read().unwrap();

    let done = Arc::new(AtomicBool::new(false));

    'outer: loop {
        trackers.par_iter().for_each(|(name, tracker)| {
            tracker.complete_one(name);

            if tracker.check_completed() {
                done.store(true, Ordering::SeqCst);
            }

            // Check if the outer loop should exit
            if done.load(Ordering::SeqCst) {
                return; // Exiting the parallel for_each early
            }

            sleep(Duration::from_millis(50));
        });

        // If we break out of the parallel loop, exit the outer loop
        if done.load(Ordering::SeqCst) {
            break 'outer;
        }
    }
}
