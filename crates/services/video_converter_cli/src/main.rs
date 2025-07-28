use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lib_core::{
    types::{
        ArgsType, AudioCodec, Converter, ConverterOptions, FfmpegOptions, HwAccel, OutputExtension,
        VideoCodec,
    },
    ProgressSystem,
};
#[cfg(feature = "embedded")]
use once_cell::sync::OnceCell;
#[cfg(feature = "embedded")]
use rust_embed::Embed;

use std::{collections::HashMap, error::Error, path::PathBuf, sync::Arc, time::Duration};
use tokio::{sync::RwLock, task::JoinSet, time::sleep};

#[cfg(feature = "embedded")]
#[derive(Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../../../binaries"]
struct Binaries;

#[cfg(feature = "embedded")]
fn ffmpeg_instance() -> &'static PathBuf {
    static INSTANCE: OnceCell<PathBuf> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        use std::fs::File;
        use std::io::Write;

        let temp_dir = std::env::temp_dir();
        let ffmpeg_path = temp_dir.join("ffmpeg.exe");

        if !ffmpeg_path.exists() {
            {
                let mut file = File::create(&ffmpeg_path).unwrap();
                file.write_all(&Binaries::get("ffmpeg.exe").unwrap().data)
                    .unwrap();
            }
        }
        ffmpeg_path
    })
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short)]
    input: PathBuf,

    #[arg(short)]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut join_set = JoinSet::new();

    let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);

    let progress_system = Arc::new(RwLock::new(ProgressSystem::new(200)));

    let mut converter =
        Converter::new_with_progress_tracker(stop_rx.clone(), progress_system.clone());

    let ffmpeg_option = FfmpegOptions::new(
        ArgsType::MatchSource,
        Some(HwAccel::Cuda),
        // None,
        ArgsType::Custom(AudioCodec::Flac),
        ArgsType::Custom(VideoCodec::H264NVENC),
        // ArgsType::Custom(VideoCodec::H264QSV),
        ArgsType::MatchSource,
        ArgsType::Custom(10000),
        ArgsType::MatchSource,
        OutputExtension::Mkv,
    );

    join_set.spawn(async move {
        converter
            .prepare_task(Arc::new(ConverterOptions::new(
                args.input,
                args.output,
                true,
                ffmpeg_option,
            )))
            .await
            .unwrap();

        #[cfg(feature = "embedded")]
        converter
            .start_conversion(Some(ffmpeg_instance()))
            .await
            .unwrap();

        #[cfg(not(feature = "embedded"))]
        converter.start_conversion(None).await.unwrap();
    });

    let progress_system_clone = progress_system.clone();
    let stop_rx_clone = stop_rx.clone();
    let stop_tx_clone = stop_tx.clone();
    join_set.spawn(async move {
        let mut bar_map = HashMap::new();
        let multi_prog = MultiProgress::new();
        while let Some(progress_list) = { progress_system_clone.write().await }.get_progress().await
        {
            let all_done = if !progress_list.is_empty() {
                Some(progress_list.iter().all(|p| p.done()))
            } else {
                None
            };

            if all_done.is_some_and(|val| val) {
                stop_tx_clone.send(true).unwrap();
                break;
            }

            if *stop_rx_clone.borrow() {
                break;
            }
            progress_list.into_iter().for_each(|progress| {
                bar_map
                    .entry(progress.folder().to_owned())
                    .and_modify(|pb: &mut ProgressBar| {
                        let pb_len = pb.length();
                        if let Some(pb_len) = pb_len {
                            if !pb_len.eq(&(progress.total() as u64)) {
                                pb.set_length(progress.total() as u64);
                            }
                            pb.set_position(progress.count() as u64);
                        }
                        pb.set_message(format!("[{}] {}", progress.folder(), progress.file()));
                    })
                    .or_insert_with(|| {
                        let mut pb = ProgressBar::new(progress.total() as u64);
                        pb = multi_prog.add(pb);
                        pb.set_style(
                            ProgressStyle::with_template(
                                "{msg:40} {bar:30.cyan/blue} {pos:>7}/{len:7}",
                            )
                            .unwrap(),
                        );
                        pb.inc(progress.count() as u64);
                        pb
                    });
            });
        }

        println!("DONE");
    });

    let stop_tx_clone = stop_tx.clone();
    join_set.spawn(async move {
        ctrlc::set_handler(move || {
            stop_tx_clone.send(true).unwrap();
        })
        .unwrap();
    });

    join_set.join_all().await;

    stop_tx.send(true).unwrap();

    drop(progress_system);

    sleep(Duration::from_secs(5)).await;

    #[cfg(feature = "embedded")]
    std::fs::remove_file(ffmpeg_instance())?;

    Ok(())
}
