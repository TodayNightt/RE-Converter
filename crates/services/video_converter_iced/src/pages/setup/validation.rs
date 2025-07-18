use crate::pages::setup::Error;
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Default)]
pub struct Validation {
    audio_bitrate_error: Option<Error>,
    video_bitrate_error: Option<Error>,
    input_dir_error: Option<Error>,
    output_dir_error: Option<Error>,
}

impl Validation {
    pub fn validate_audio_bitrate(&mut self, value: Option<String>) -> Option<u32> {
        let audio_bitrate = value?;

        if audio_bitrate.is_empty() {
            return Some(0);
        }

        match audio_bitrate.parse() {
            Ok(val) => {
                self.audio_bitrate_error = None;
                Some(val)
            }
            Err(_) => {
                self.audio_bitrate_error = Some(Error::AudioBitrateParseError);
                None
            }
        }
    }

    pub fn validate_video_bitrate(&mut self, value: Option<String>) -> Option<u32> {
        let video_bitrate = value?;

        if video_bitrate.is_empty() {
            return Some(0);
        }

        match video_bitrate.parse() {
            Ok(val) => {
                self.video_bitrate_error = None;
                Some(val)
            }
            Err(_) => {
                self.video_bitrate_error = Some(Error::VideoBitrateParseError);
                None
            }
        }
    }

    pub fn validate_input_dir(&mut self, value: Option<PathBuf>) -> Option<PathBuf> {
        if value.is_none() {
            self.input_dir_error = Some(Error::InputDirEmpty);
            return None;
        }
        value
    }

    pub fn validate_output_dir(&mut self, value: Option<PathBuf>) -> Option<PathBuf> {
        if value.is_none() {
            self.output_dir_error = Some(Error::OutputDirEmpty);
            return None;
        }
        value
    }
}

impl Validation {
    pub fn audio_bitrate_error(&self) -> Option<Error> {
        self.audio_bitrate_error
    }

    pub fn video_bitrate_error(&self) -> Option<Error> {
        self.video_bitrate_error
    }

    pub fn input_dir_error(&self) -> Option<Error> {
        self.input_dir_error
    }

    pub fn output_dir_error(&self) -> Option<Error> {
        self.output_dir_error
    }
}
