use std::{fmt::Display, path::PathBuf};
use typeshare::typeshare;

use lib_utils::arg::Arg;
use serde::{Deserialize, Serialize};

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Resolution {
    R720P,
    R1080P,
    R1440P,
    R4K,
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resolution::R720P => f.write_str("1280x720"),
            Resolution::R1080P => f.write_str("1920x1080"),
            Resolution::R1440P => f.write_str("2560x1440"),
            Resolution::R4K => f.write_str("4096x2160"),
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum AudioCodec {
    Flac,
    Aac,
    Ipcm,
}

impl Display for AudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioCodec::Flac => f.write_str("flac"),
            AudioCodec::Ipcm => f.write_str("pcm_s24be"),
            AudioCodec::Aac => f.write_str("aac"),
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PictureFormat {
    Pf42210B,
    Pf4228B,
    Pf42010B,
    Pf4208B,
}

impl Display for PictureFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PictureFormat::Pf42210B => f.write_str("yuv422p10le"),
            PictureFormat::Pf4228B => f.write_str("yuv422p"),
            PictureFormat::Pf42010B => f.write_str("yuv420p10le"),
            PictureFormat::Pf4208B => f.write_str("yuv420p"),
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum VideoCodec {
    H264,
    H264NVENC,
    H264AMF,
    H264QSV,
    H265,
    H265NVENC,
    H265AMF,
    H265QSV,
    CineForm,
    Prores,
}
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum OutputExtension {
    Default,
    Mkv,
    Mov,
    Mp4,
    Mp3,
}

impl Display for OutputExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputExtension::Default | OutputExtension::Mkv => f.write_str("mkv"),
            OutputExtension::Mp4 => f.write_str("mp4"),
            OutputExtension::Mov => f.write_str("mov"),
            OutputExtension::Mp3 => f.write_str("mp3"),
        }
    }
}

impl Display for VideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoCodec::H264 => f.write_str("libx264"),
            VideoCodec::H264NVENC => f.write_str("h264_nvenc"),
            VideoCodec::H264AMF => f.write_str("h264_amf"),
            VideoCodec::H264QSV => f.write_str("h264_qsv"),
            VideoCodec::H265 => f.write_str("libx265"),
            VideoCodec::H265NVENC => f.write_str("hevc_nvenc"),
            VideoCodec::H265AMF => f.write_str("hevc_amf"),
            VideoCodec::H265QSV => f.write_str("hevc_qsv"),
            VideoCodec::CineForm => f.write_str("cfhd"),
            VideoCodec::Prores => f.write_str("prores"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[typeshare]
#[serde(tag = "type", content = "content")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ArgsType<T> {
    MatchSource,
    Custom(T),
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HwAccel {
    Cuda,
    Directx,
    Vaapi,
    Vulkan,
}
impl Display for HwAccel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cuda => f.write_str("cuda"),
            Self::Directx => f.write_str("d3d11va"),
            Self::Vaapi => f.write_str("vaapi"),
            Self::Vulkan => f.write_str("vulkan"),
        }
    }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FfmpegOptions {
    pub resolution: ArgsType<Resolution>,
    pub hwaccel: Option<HwAccel>,
    pub audio_codec: ArgsType<AudioCodec>,
    pub video_codec: ArgsType<VideoCodec>,
    pub audio_bitrate: ArgsType<u32>,
    pub video_bitrate: ArgsType<u32>,
    pub picture_format: ArgsType<PictureFormat>,
    pub output_extension: OutputExtension,
}

impl FfmpegOptions {
    pub fn build(self, input: PathBuf, output: PathBuf) -> Vec<String> {
        let mut args = Vec::new();

        if let Some(hwaccel) = self.hwaccel {
            args.extend(Arg::new("hwaccel").value(hwaccel.to_string()).build());

            if let ArgsType::Custom(video_codec) = self.video_codec {
                if matches!(video_codec, VideoCodec::H264NVENC) {
                    args.extend(Arg::new("hwaccel_output_format").value("cuda").build());
                }
            }
        }

        args.extend(Arg::new("i").value(input.to_str().unwrap()).build());

        if !matches!(self.resolution, ArgsType::MatchSource) {
            args.extend(
                Arg::new("vf")
                    .value(
                        Arg::new(
                            Arg::new("scale")
                                .without_dash()
                                .value(self.resolution.to_string())
                                .with_value_spacer("=   ")
                                .build()
                                .join(""),
                        )
                        .without_dash()
                        .value(
                            Arg::new("flags")
                                .without_dash()
                                .value("lanczos")
                                .with_value_spacer("=")
                                .build()
                                .join(""),
                        )
                        .with_value_spacer(":")
                        .build()
                        .join(""),
                    )
                    .build(),
            );
        }

        //Vidoe codec and bitrate
        match (self.video_codec, self.video_bitrate) {
            (ArgsType::Custom(codec), ArgsType::Custom(bitrate)) => {
                args.extend(Arg::new("c:v").value(codec.to_string()).build());

                args.extend(Arg::new("b:v").value(bitrate.to_string() + "k").build());
            }
            (ArgsType::MatchSource, ArgsType::MatchSource) => {
                args.extend(Arg::new("c:v").value(self.video_codec.to_string()).build());
            }
            (_, ArgsType::Custom(bitrate)) => {
                args.extend(Arg::new("b:v").value(bitrate.to_string() + "k").build());
            }
            (ArgsType::Custom(codec), _) => {
                args.extend(Arg::new("c:v").value(codec.to_string()).build());
            }
        }

        //Audio codec and bitrate
        match (self.audio_codec, self.audio_bitrate) {
            (ArgsType::Custom(codec), ArgsType::Custom(bitrate)) => {
                args.extend(Arg::new("c:a").value(codec.to_string()).build());

                args.extend(Arg::new("b:a").value(bitrate.to_string() + "k").build());
            }
            (ArgsType::MatchSource, ArgsType::MatchSource) => {
                args.extend(Arg::new("c:a").value("copy").build());
            }
            (_, ArgsType::Custom(bitrate)) => {
                args.extend(Arg::new("b:a").value(bitrate.to_string() + "k").build());
            }
            (ArgsType::Custom(codec), _) => {
                args.extend(Arg::new("c:a").value(codec.to_string()).build());
            }
        }

        args.push(output.to_str().unwrap().to_string());

        args
    }
}

#[typeshare]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConverterOptions {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub need_sorting: bool,
    pub ffmpeg_options: FfmpegOptions,
}

impl Display for ArgsType<AudioCodec> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsType::MatchSource => f.write_str("copy"),
            ArgsType::Custom(codec) => f.write_str(&codec.to_string()),
        }
    }
}

impl Display for ArgsType<VideoCodec> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsType::MatchSource => f.write_str("copy"),
            ArgsType::Custom(codec) => f.write_str(&codec.to_string()),
        }
    }
}

impl Display for ArgsType<PictureFormat> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsType::MatchSource => f.write_str("copy"),
            ArgsType::Custom(custom) => f.write_str(&custom.to_string()),
        }
    }
}
impl Display for ArgsType<Resolution> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsType::MatchSource => f.write_str(""),
            ArgsType::Custom(res) => f.write_str(&res.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{path::PathBuf, str::FromStr};

    use crate::converter::options::Resolution;

    use super::{FfmpegOptions, HwAccel};

    #[test]
    fn test_ffmpeg_build() {
        let options = FfmpegOptions {
            audio_bitrate: super::ArgsType::MatchSource,
            video_bitrate: super::ArgsType::MatchSource,
            resolution: super::ArgsType::Custom(Resolution::R1080P),
            hwaccel: Some(HwAccel::Cuda),
            video_codec: super::ArgsType::Custom(super::VideoCodec::H264NVENC),
            audio_codec: super::ArgsType::Custom(super::AudioCodec::Flac),
            picture_format: super::ArgsType::MatchSource,
            output_extension: super::OutputExtension::Default,
        };

        let args = options.build(
            PathBuf::from_str("/s/video/a.mp4").unwrap(),
            PathBuf::from_str("/s/video/a.mkv").unwrap(),
        );

        assert_eq!(
            args,
            vec![
                "-hwaccel",
                "cuda",
                "-hwaccel_output_format",
                "cuda",
                "-i",
                "/s/video/a.mp4",
                "-vf",
                "\"scale:1920x1080:flags=lanczos\"",
                "-c:v",
                "h264_nvenc",
                "-c:v",
                "flac",
                "/s/video/a.mkv"
            ]
        )
    }
}
