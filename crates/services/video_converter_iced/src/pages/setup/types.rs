use crate::utils::EnumToArray;
use lib_core::types::ArgsType;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum AudioCodec {
    #[default]
    Flac,
    Aac,
    Ipcm,
}
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum VideoCodec {
    #[default]
    H264,
    H265,
    CineForm,
    Prores,
}
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum HwAccel {
    #[default]
    None,
    Cuda,
    Directx,
    Vaapi,
    Vulkan,
}

impl EnumToArray<5> for HwAccel {
    type T = HwAccel;
    fn all() -> [Self::T; 5] {
        [
            HwAccel::None,
            HwAccel::Cuda,
            HwAccel::Directx,
            HwAccel::Vaapi,
            HwAccel::Vulkan,
        ]
    }
}

impl From<HwAccel> for String {
    fn from(value: HwAccel) -> Self {
        let val = match value {
            HwAccel::Cuda => "cuda",
            HwAccel::Directx => "directX",
            HwAccel::Vaapi => "vaapi",
            HwAccel::Vulkan => "vulkan",
            HwAccel::None => "none",
        };

        val.to_string()
    }
}
#[derive(Debug, Clone, Default, Copy, Eq, PartialEq)]
pub enum OutputExtension {
    #[default]
    Mkv,
    Mov,
    Mp4,
    Mp3,
}

impl EnumToArray<4> for OutputExtension {
    type T = OutputExtension;

    fn all() -> [Self::T; 4] {
        [
            OutputExtension::Mkv,
            OutputExtension::Mov,
            OutputExtension::Mp3,
            OutputExtension::Mp4,
        ]
    }
}

impl From<OutputExtension> for String {
    fn from(value: OutputExtension) -> Self {
        let val = match value {
            OutputExtension::Mkv => "mkv",
            OutputExtension::Mp3 => "mp3",
            OutputExtension::Mp4 => "mp4",
            OutputExtension::Mov => "mov",
        };

        val.to_string()
    }
}

impl EnumToArray<4> for VideoCodec {
    type T = VideoCodec;
    fn all() -> [Self::T; 4] {
        [
            VideoCodec::H264,
            VideoCodec::H265,
            VideoCodec::CineForm,
            VideoCodec::Prores,
        ]
    }
}

impl From<VideoCodec> for String {
    fn from(value: VideoCodec) -> Self {
        let val = match value {
            VideoCodec::H264 => "h264",
            VideoCodec::CineForm => "cineform",
            VideoCodec::H265 => "h265",
            VideoCodec::Prores => "prores",
        };

        val.to_string()
    }
}

impl EnumToArray<3> for AudioCodec {
    type T = AudioCodec;
    fn all() -> [Self::T; 3] {
        [AudioCodec::Flac, AudioCodec::Aac, AudioCodec::Ipcm]
    }
}

impl From<AudioCodec> for String {
    fn from(value: AudioCodec) -> Self {
        let val = match value {
            AudioCodec::Flac => "flac",
            AudioCodec::Aac => "aac",
            AudioCodec::Ipcm => "ipcm",
        };

        val.to_string()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ToggleType {
    AC,
    VC,
    AB,
    VB,
    OEX,
}

impl From<HwAccel> for Option<lib_core::types::HwAccel> {
    fn from(value: HwAccel) -> Self {
        match value {
            HwAccel::None => None,
            HwAccel::Cuda => Some(lib_core::types::HwAccel::Cuda),
            HwAccel::Vulkan => Some(lib_core::types::HwAccel::Vulkan),
            HwAccel::Vaapi => Some(lib_core::types::HwAccel::Vaapi),
            HwAccel::Directx => Some(lib_core::types::HwAccel::Directx),
        }
    }
}

impl From<AudioCodec> for ArgsType<lib_core::types::AudioCodec> {
    fn from(value: AudioCodec) -> Self {
        match value {
            AudioCodec::Flac => ArgsType::Custom(lib_core::types::AudioCodec::Flac),
            AudioCodec::Ipcm => ArgsType::Custom(lib_core::types::AudioCodec::Ipcm),
            AudioCodec::Aac => ArgsType::Custom(lib_core::types::AudioCodec::Aac),
        }
    }
}

impl From<VideoCodec> for ArgsType<lib_core::types::VideoCodec> {
    fn from(value: VideoCodec) -> Self {
        match value {
            VideoCodec::H264 => ArgsType::Custom(lib_core::types::VideoCodec::H264),
            VideoCodec::H265 => ArgsType::Custom(lib_core::types::VideoCodec::H265),
            VideoCodec::CineForm => ArgsType::Custom(lib_core::types::VideoCodec::CineForm),
            VideoCodec::Prores => ArgsType::Custom(lib_core::types::VideoCodec::Prores),
        }
    }
}

impl From<OutputExtension> for lib_core::types::OutputExtension {
    fn from(value: OutputExtension) -> Self {
        match value {
            OutputExtension::Mov => lib_core::types::OutputExtension::Mov,
            OutputExtension::Mp4 => lib_core::types::OutputExtension::Mp4,
            OutputExtension::Mkv => lib_core::types::OutputExtension::Mkv,
            OutputExtension::Mp3 => lib_core::types::OutputExtension::Mp3,
        }
    }
}

impl From<lib_core::types::OutputExtension> for OutputExtension {
    fn from(value: lib_core::types::OutputExtension) -> Self {
        match value {
            lib_core::types::OutputExtension::Mov => OutputExtension::Mov,
            lib_core::types::OutputExtension::Mp4 => OutputExtension::Mp4,
            lib_core::types::OutputExtension::Mkv => OutputExtension::Mkv,
            lib_core::types::OutputExtension::Mp3 => OutputExtension::Mp3,
            _ => OutputExtension::Mkv,
        }
    }
}

impl From<lib_core::types::VideoCodec> for VideoCodec {
    fn from(value: lib_core::types::VideoCodec) -> Self {
        use lib_core::types::VideoCodec as Core;
        match value {
            Core::H264 | Core::H264NVENC | Core::H264QSV | Core::H264AMF => VideoCodec::H264,
            Core::H265 | Core::H265NVENC | Core::H265QSV | Core::H265AMF => VideoCodec::H265,
            Core::CineForm => VideoCodec::CineForm,
            Core::Prores => VideoCodec::Prores,
            _ => VideoCodec::H264,
        }
    }
}

impl From<lib_core::types::AudioCodec> for AudioCodec {
    fn from(value: lib_core::types::AudioCodec) -> Self {
        use lib_core::types::AudioCodec as Core;
        match value {
            Core::Aac => AudioCodec::Aac,
            Core::Flac => AudioCodec::Flac,
            Core::Ipcm => AudioCodec::Ipcm,
            _ => AudioCodec::Flac,
        }
    }
}

impl From<lib_core::types::HwAccel> for HwAccel {
    fn from(value: lib_core::types::HwAccel) -> Self {
        use lib_core::types::HwAccel as Core;
        match value {
            Core::Cuda => HwAccel::Cuda,
            Core::Vulkan => HwAccel::Vulkan,
            Core::Vaapi => HwAccel::Vaapi,
            Core::Directx => HwAccel::Directx,
            _ => HwAccel::None,
        }
    }
}
