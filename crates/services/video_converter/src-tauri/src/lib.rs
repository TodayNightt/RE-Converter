// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// mod config;
// mod error;
// mod websocket_server;

use std::sync::Arc;

// use config::Config;
// pub use error::{Error, Result};
use lib_core::ConverterOptions;

use tauri::{Emitter, Listener, Manager};
use tokio::sync::RwLock;

// struct State {
//     config: Config,
// }

// #[tauri::command]
// fn get_last_saved(state: tauri::State<State>) -> Option<ConverterOptions> {
// state.config.last_saved().clone()

// use std::{fmt::Display, path::PathBuf};
// use typeshare::typeshare;

// use lib_utils::arg::Arg;
// use serde::{Deserialize, Serialize};

// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum Resolution {
//     R720P,
//     R1080P,
//     R1440P,
//     R4K,
// }

// impl Display for Resolution {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Resolution::R720P => f.write_str("1280x720"),
//             Resolution::R1080P => f.write_str("1920x1080"),
//             Resolution::R1440P => f.write_str("2560x1440"),
//             Resolution::R4K => f.write_str("4096x2160"),
//         }
//     }
// }

// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// #[non_exhaustive]
// pub enum AudioCodec {
//     Flac,
//     Aac,
//     Ipcm,
// }

// impl Display for AudioCodec {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             AudioCodec::Flac => f.write_str("flac"),
//             AudioCodec::Ipcm => f.write_str("pcm_s24be"),
//             AudioCodec::Aac => f.write_str("aac"),
//         }
//     }
// }

// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum PictureFormat {
//     Pf42210B,
//     Pf4228B,
//     Pf42010B,
//     Pf4208B,
// }

// impl Display for PictureFormat {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             PictureFormat::Pf42210B => f.write_str("yuv422p10le"),
//             PictureFormat::Pf4228B => f.write_str("yuv422p"),
//             PictureFormat::Pf42010B => f.write_str("yuv420p10le"),
//             PictureFormat::Pf4208B => f.write_str("yuv420p"),
//         }
//     }
// }

// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[non_exhaustive]
// #[serde(rename_all = "camelCase")]
// pub enum VideoCodec {
//     H264,
//     H264NVENC,
//     H264AMF,
//     H264QSV,
//     H265,
//     H265NVENC,
//     H265AMF,
//     H265QSV,
//     CineForm,
//     Prores,
// }
// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[non_exhaustive]
// #[serde(rename_all = "camelCase")]
// pub enum OutputExtension {
//     Default,
//     Mkv,
//     Mov,
//     Mp4,
//     Mp3,
// }

// impl Display for OutputExtension {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             OutputExtension::Default | OutputExtension::Mkv => f.write_str("mkv"),
//             OutputExtension::Mp4 => f.write_str("mp4"),
//             OutputExtension::Mov => f.write_str("mov"),
//             OutputExtension::Mp3 => f.write_str("mp3"),
//         }
//     }
// }

// impl Display for VideoCodec {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             VideoCodec::H264 => f.write_str("libx264"),
//             VideoCodec::H264NVENC => f.write_str("h264_nvenc"),
//             VideoCodec::H264AMF => f.write_str("h264_amf"),
//             VideoCodec::H264QSV => f.write_str("h264_qsv"),
//             VideoCodec::H265 => f.write_str("libx265"),
//             VideoCodec::H265NVENC => f.write_str("hevc_nvenc"),
//             VideoCodec::H265AMF => f.write_str("hevc_amf"),
//             VideoCodec::H265QSV => f.write_str("hevc_qsv"),
//             VideoCodec::CineForm => f.write_str("cfhd"),
//             VideoCodec::Prores => f.write_str("prores"),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[typeshare]
// #[serde(tag = "type", content = "content")]
// #[serde(rename_all = "camelCase")]
// #[non_exhaustive]
// pub enum ArgsType<T> {
//     MatchSource,
//     Custom(T),
// }

// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub enum HwAccel {
//     Cuda,
//     Directx,
//     Vaapi,
//     Vulkan,
// }
// impl Display for HwAccel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Cuda => f.write_str("cuda"),
//             Self::Directx => f.write_str("d3d11va"),
//             Self::Vaapi => f.write_str("vaapi"),
//             Self::Vulkan => f.write_str("vulkan"),
//         }
//     }
// }

// #[typeshare]
// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct FfmpegOptions {
//     resolution: ArgsType<Resolution>,
//     hwaccel: Option<HwAccel>,
//     audio_codec: ArgsType<AudioCodec>,
//     video_codec: ArgsType<VideoCodec>,
//     audio_bitrate: ArgsType<u32>,
//     video_bitrate: ArgsType<u32>,
//     picture_format: ArgsType<PictureFormat>,
//     pub output_extension: OutputExtension,
// }

// impl FfmpegOptions {
//     pub fn build(self, input: PathBuf, output: PathBuf) -> Vec<String> {
//         let mut args = Vec::new();

//         if let Some(hwaccel) = self.hwaccel {
//             args.extend(Arg::new("hwaccel").value(hwaccel.to_string()).build());

//             if let ArgsType::Custom(video_codec) = self.video_codec {
//                 if matches!(video_codec, VideoCodec::H264NVENC) {
//                     args.extend(Arg::new("hwaccel_output_format").value("cuda").build());
//                 }
//             }
//         }

//         args.extend(Arg::new("i").value(input.to_str().unwrap()).build());

//         if !matches!(self.resolution, ArgsType::MatchSource) {
//             args.extend(
//                 Arg::new("vf")
//                     .value(
//                         Arg::new(
//                             Arg::new("scale")
//                                 .without_dash()
//                                 .value(self.resolution.to_string())
//                                 .with_value_spacer("=   ")
//                                 .build()
//                                 .join(""),
//                         )
//                         .without_dash()
//                         .value(
//                             Arg::new("flags")
//                                 .without_dash()
//                                 .value("lanczos")
//                                 .with_value_spacer("=")
//                                 .build()
//                                 .join(""),
//                         )
//                         .with_value_spacer(":")
//                         .build()
//                         .join(""),
//                     )
//                     .build(),
//             );
//         }

//         //Vidoe codec and bitrate
//         match (self.video_codec, self.video_bitrate) {
//             (ArgsType::Custom(codec), ArgsType::Custom(bitrate)) => {
//                 args.extend(Arg::new("c:v").value(codec.to_string()).build());

//                 args.extend(Arg::new("b:v").value(bitrate.to_string() + "k").build());
//             }
//             (ArgsType::MatchSource, ArgsType::MatchSource) => {
//                 args.extend(Arg::new("c:v").value(self.video_codec.to_string()).build());
//             }
//             (_, ArgsType::Custom(bitrate)) => {
//                 args.extend(Arg::new("b:v").value(bitrate.to_string() + "k").build());
//             }
//             (ArgsType::Custom(codec), _) => {
//                 args.extend(Arg::new("c:v").value(codec.to_string()).build());
//             }
//         }

//         //Audio codec and bitrate
//         match (self.audio_codec, self.audio_bitrate) {
//             (ArgsType::Custom(codec), ArgsType::Custom(bitrate)) => {
//                 args.extend(Arg::new("c:a").value(codec.to_string()).build());

//                 args.extend(Arg::new("b:a").value(bitrate.to_string() + "k").build());
//             }
//             (ArgsType::MatchSource, ArgsType::MatchSource) => {
//                 args.extend(Arg::new("c:a").value("copy").build());
//             }
//             (_, ArgsType::Custom(bitrate)) => {
//                 args.extend(Arg::new("b:a").value(bitrate.to_string() + "k").build());
//             }
//             (ArgsType::Custom(codec), _) => {
//                 args.extend(Arg::new("c:a").value(codec.to_string()).build());
//             }
//         }

//         args.push(output.to_str().unwrap().to_string());

//         args
//     }
// }

// #[typeshare]
// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ConverterOptions {
//     pub input_dir: PathBuf,
//     pub output_dir: PathBuf,
//     pub need_sorting: bool,
//     pub ffmpeg_options: FfmpegOptions,
// }

// impl Display for ArgsType<AudioCodec> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ArgsType::MatchSource => f.write_str("copy"),
//             ArgsType::Custom(codec) => f.write_str(&codec.to_string()),
//         }
//     }
// }

// impl Display for ArgsType<VideoCodec> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ArgsType::MatchSource => f.write_str("copy"),
//             ArgsType::Custom(codec) => f.write_str(&codec.to_string()),
//         }
//     }
// }

// impl Display for ArgsType<PictureFormat> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ArgsType::MatchSource => f.write_str("copy"),
//             ArgsType::Custom(custom) => f.write_str(&custom.to_string()),
//         }
//     }
// }
// impl Display for ArgsType<Resolution> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ArgsType::MatchSource => f.write_str(""),
//             ArgsType::Custom(res) => f.write_str(&res.to_string()),
//         }
//     }
// }

// Some(Config {
//     last_saved: Some(ConverterOptions {
//         input_dir: "input".into(),
//         output_dir: "output".into(),
//         need_sorting: false,
//         ffmpeg_options: FfmpegOptions {
//             resolution: ArgsType::Custom(Resolution::R720P),
//             hwaccel: Some(HwAccel::Cuda),
//             audio_codec: ArgsType::Custom(AudioCodec::Aac),
//             video_codec: ArgsType::Custom(VideoCodec::H264),
//             audio_bitrate: ArgsType::Custom(128),
//             video_bitrate: ArgsType::Custom(5000),
//             picture_format: ArgsType::Custom(PictureFormat::Pf42210B),
//             output_extension: OutputExtension::Mp4,
//         },
//     }),
//     saved_path: "config.json".into(),
// })
//     state.config.last_saved().clone()
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        // .manage(State {
        //     config: Config::default(),
        // })
        // .setup(|app| {
        //     let mut config_dir = app.path().app_data_dir()?;
        //
        //     config_dir.push("config.json");
        //     let config = Config::load(&config_dir)?;
        //
        //     let state = Arc::new(RwLock::new(State { config }));
        //
        //     let state_clone = Arc::clone(&state);
        //
        //     app.manage(state);
        //
        //     let websocket_task = tauri::async_runtime::spawn(async move {
        //         websocket_server::start_server(state_clone).await
        //     });
        //     let handle_clone = Arc::new(app.handle().clone());
        //     app.listen_any("tauri://close-requested", move |_| {
        //         println!("Closing websocket server");
        //         websocket_task.abort(); // Abort WebSocket task
        //         handle_clone.exit(0);
        //     });
        //
        //     let handle = Arc::new(app.handle().clone());
        //     // Check if FFmpeg is installed
        //     let ffmpeg_installed = std::process::Command::new("ffmpeg")
        //         .arg("-version")
        //         .output()
        //         .map(|output| output.status.success())
        //         .unwrap_or(false);
        //
        //     app.listen_any("tauri://window-created", move |_| {
        //         if !ffmpeg_installed {
        //             println!("FFmpeg not installed, emitting event...");
        //             handle.emit("no-ffmpeg", ()).unwrap();
        //         } else {
        //             println!("FFmpeg is installed.");
        //         }
        //     });
        //
        //     Ok(())
        // })
        // .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_websocket::init())
        // .invoke_handler(tauri::generate_handler![get_last_saved])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
