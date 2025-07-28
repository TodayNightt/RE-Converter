use iced::widget::{
    Column, button, column, container, horizontal_space, radio, rich_text, row, scrollable, span,
    text, text_input, toggler,
};
use iced::{Alignment, Color, Element, Font, Length, Radians, Rotation, Subscription, Task, font};

use std::{path::PathBuf, sync::Arc};
use tokio::sync::{RwLock, watch};

use lib_core::{
    ProgressSystem,
    types::{ArgsType, Converter, ConverterOptions, FfmpegOptions},
};

use crate::{
    Message,
    assets::svg::back_arrow,
    pages::{
        Page, Pages,
        progress::{ProgressPage, ProgressPageMessage},
        setup::{
            types::{AudioCodec, HwAccel, OutputExtension, ToggleType, VideoCodec},
            validation::Validation,
        },
    },
    radios,
    utils::{
        EnumToArray,
        extensions::{OptionStringExt, OptionValueExt},
    },
};

use crate::config::Config;
pub use error::Error;

#[cfg(feature = "embedded")]
use crate::assets::ffmpeg_instance;

mod types;

mod validation;

#[derive(Debug, Clone, Default)]
pub struct SetupPage {
    input_dir: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    audio_bitrate: Option<u32>,
    video_bitrate: Option<u32>,
    audio_codec: Option<AudioCodec>,
    video_codec: Option<VideoCodec>,
    hw_accel: HwAccel,
    output_extension: Option<OutputExtension>,
    debug: bool,
    validation: Validation,
    converting_page_state: Option<ProgressPage>,
}

#[derive(Debug, Clone)]
pub enum FolderIden {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub(crate) enum SetupPageMessage {
    DefaultValue(Option<ConverterOptions>),
    SelectFolder(FolderIden),
    InputFolder(Option<PathBuf>),
    OutputFolder(Option<PathBuf>),
    AudioCodecChange(Option<AudioCodec>),
    VideoCodecChange(Option<VideoCodec>),
    AudioBitrateChange(Option<String>),
    VideoBirateChange(Option<String>),
    OutputExtensionChange(Option<OutputExtension>),
    HwAccelChange(HwAccel),
    ChangeToProgressPage(Option<(watch::Sender<bool>, Arc<RwLock<ProgressSystem>>)>),
    Convert,
    UpdateConfigSettings(ConverterOptions),
    EnableToggle(ToggleType, bool),
    Noop,
    Debug,
}

mod error {
    use std::fmt::Formatter;

    #[derive(Debug, Clone, Copy)]
    pub enum Error {
        InputDirEmpty,
        OutputDirEmpty,
        AudioBitrateParseError,
        VideoBitrateParseError,
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::AudioBitrateParseError | Error::VideoBitrateParseError => {
                    f.write_str("Only numbers are allowed")
                }
                _ => f.write_str("Internal Error"),
            }
        }
    }

    impl core::error::Error for Error {}
}

impl Page for SetupPage {
    fn update(&mut self, message: Message) -> (Task<Message>, Option<Box<Pages>>) {
        if let Message::SetupPage(message) = message {
            match message {
                SetupPageMessage::DefaultValue(op) => {
                    if let Some(options) = op {
                        return (
                            Task::none(),
                            Some(Box::new(Pages::Setup(SetupPage::new(options)))),
                        );
                    }
                }
                SetupPageMessage::InputFolder(path) => self.input_dir = path,
                SetupPageMessage::OutputFolder(path) => self.output_dir = path,
                SetupPageMessage::SelectFolder(FolderIden::Input) => {
                    return (
                        Task::perform(select_folder(), |res| {
                            Message::SetupPage(SetupPageMessage::InputFolder(res))
                        }),
                        None,
                    );
                }
                SetupPageMessage::SelectFolder(FolderIden::Output) => {
                    return (
                        Task::perform(select_folder(), |res| {
                            Message::SetupPage(SetupPageMessage::OutputFolder(res))
                        }),
                        None,
                    );
                }
                SetupPageMessage::AudioCodecChange(ac) => self.audio_codec = ac,
                SetupPageMessage::VideoCodecChange(vc) => self.video_codec = vc,
                SetupPageMessage::AudioBitrateChange(ab) => {
                    self.audio_bitrate = self.validation.validate_audio_bitrate(ab);
                }
                SetupPageMessage::VideoBirateChange(vb) => {
                    self.video_bitrate = self.validation.validate_video_bitrate(vb);
                }
                SetupPageMessage::OutputExtensionChange(val) => self.output_extension = val,
                SetupPageMessage::HwAccelChange(val) => self.hw_accel = val,
                SetupPageMessage::Convert => {
                    let input_dir = self.validation.validate_input_dir(self.input_dir.clone());
                    let output_dir = self.validation.validate_output_dir(self.output_dir.clone());
                    let (Some(input_dir), Some(output_dir)) = (input_dir, output_dir) else {
                        return (Task::none(), None);
                    };

                    let (task, stop_signal, progress_system, converter_opts) =
                        self.convert(input_dir, output_dir);

                    return (
                        Task::done(Message::SetupPage(SetupPageMessage::UpdateConfigSettings(
                            converter_opts,
                        )))
                        .chain(Task::done(Message::SetupPage(
                            SetupPageMessage::ChangeToProgressPage(Some((
                                stop_signal,
                                progress_system,
                            ))),
                        )))
                        .chain(task),
                        None,
                    );
                }
                SetupPageMessage::UpdateConfigSettings(opts) => {
                    return (
                        Task::future(async move {
                            { Config::get_instance().write().await }
                                .update_last_saved_and_save(opts)
                                .unwrap();
                            Message::SetupPage(SetupPageMessage::Noop)
                        }),
                        None,
                    );
                }
                SetupPageMessage::ChangeToProgressPage(Some((stop_tx, progress_system))) => {
                    return (
                        Task::none(),
                        Some(Box::new(Pages::Progress(ProgressPage::new(
                            stop_tx,
                            progress_system,
                        )))),
                    );
                }

                SetupPageMessage::ChangeToProgressPage(None) => {
                    if let Some(progress_page) = self.converting_page_state.as_ref() {
                        return (
                            Task::none(),
                            Some(Box::new(Pages::Progress(progress_page.clone()))),
                        );
                    }
                }
                SetupPageMessage::EnableToggle(tt, b) => match tt {
                    ToggleType::AC => {
                        self.audio_codec = if b { Some(AudioCodec::default()) } else { None }
                    }

                    ToggleType::VC => {
                        self.video_codec = if b { Some(VideoCodec::default()) } else { None }
                    }

                    ToggleType::AB => {
                        self.audio_bitrate = if b { Some(10000) } else { None };
                    }
                    ToggleType::VB => {
                        self.video_bitrate = if b { Some(10000) } else { None };
                    }
                    ToggleType::OEX => {
                        self.output_extension = if b {
                            Some(OutputExtension::default())
                        } else {
                            None
                        }
                    }
                },
                SetupPageMessage::Debug => self.debug = !self.debug,
                SetupPageMessage::Noop => {}
            }
        }
        (Task::none(), None)
    }

    fn view(&self) -> Element<Message> {
        let folder_selector_input =
            SetupPage::create_folder_selector(FolderIden::Input, self.input_dir.as_ref(), "Input");

        let folder_selector_output = SetupPage::create_folder_selector(
            FolderIden::Output,
            self.output_dir.as_ref(),
            "Output",
        );

        let audio_codec = SetupPage::create_toggler_element(
            self.audio_codec,
            "Audio Codec",
            radios!(
                AudioCodec::all(),
                self.audio_codec,
                |val| Message::SetupPage(SetupPageMessage::AudioCodecChange(Some(val))),
                row,
                10
            )
            .into(),
            ToggleType::AC,
        );

        let video_codec = SetupPage::create_toggler_element(
            self.video_codec,
            "Video Codec",
            radios!(
                VideoCodec::all(),
                self.video_codec,
                |val| Message::SetupPage(SetupPageMessage::VideoCodecChange(Some(val))),
                row,
                10
            )
            .into(),
            ToggleType::VC,
        );

        let audio_bitrate = SetupPage::create_toggler_element(
            self.audio_bitrate,
            "Audio Bitrate",
            row![
                text_input("10000k", &self.audio_bitrate.unwrap_or_empty_string())
                    .on_input(|val| {
                        let val = val.parse().unwrap();
                        Message::SetupPage(SetupPageMessage::AudioBitrateChange(Some(val)))
                    })
                    .width(Length::FillPortion(3)),
                text(
                    self.validation
                        .audio_bitrate_error()
                        .unwrap_or_empty_string()
                )
                .style(text::danger)
                .font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .width(Length::FillPortion(3)),
            ]
            .spacing(5)
            .into(),
            ToggleType::AB,
        );

        let video_bitrate = SetupPage::create_toggler_element(
            self.video_bitrate,
            "Video Bitrate",
            row![
                text_input("10000k", &self.video_bitrate.unwrap_or_empty_string())
                    .on_input(|val| {
                        let val = val.parse().unwrap();
                        Message::SetupPage(SetupPageMessage::VideoBirateChange(Some(val)))
                    })
                    .width(Length::FillPortion(3)),
                text(
                    self.validation
                        .video_bitrate_error()
                        .unwrap_or_empty_string()
                )
                .style(text::danger)
                .font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .width(Length::FillPortion(3)),
            ]
            .spacing(5)
            .into(),
            ToggleType::VB,
        );

        let output_extension = SetupPage::create_toggler_element(
            self.output_extension,
            "Output Extension",
            radios!(
                OutputExtension::all(),
                self.output_extension,
                |val| Message::SetupPage(SetupPageMessage::OutputExtensionChange(Some(val))),
                row,
                10
            )
            .into(),
            ToggleType::OEX,
        );

        let hw_a = container(row![
            container(
                text("HwAccel")
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center()
                    .font(Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })
            )
            .width(Length::FillPortion(2)),
            radios!(
                HwAccel::all(),
                Some(self.hw_accel),
                |val| Message::SetupPage(SetupPageMessage::HwAccelChange(val)),
                row,
                10
            )
            .padding(10)
            .width(Length::FillPortion(3))
            .height(Length::Fill)
            .align_y(Alignment::Center),
        ])
        .center_y(Length::Fixed(135.))
        .style(container::bordered_box)
        .style(container::rounded_box);

        let convert_btn = button(
            text!("Convert")
                .size(20)
                .font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .center(),
        )
        .on_press(Message::SetupPage(SetupPageMessage::Convert))
        .padding(10)
        .style(button::primary);

        let controls = match self.converting_page_state {
            Some(_) => {
                row![
                    horizontal_space(),
                    button(row![
                        "Progress",
                        back_arrow().rotation(Rotation::Solid(Radians(std::f32::consts::PI))),
                    ])
                    .on_press(Message::SetupPage(SetupPageMessage::ChangeToProgressPage(
                        None
                    ),))
                    .style(button::text)
                ]
            }
            None => row![horizontal_space()],
        };

        let content: Element<_> = column![
            folder_selector_input,
            folder_selector_output,
            audio_codec,
            video_codec,
            audio_bitrate,
            video_bitrate,
            output_extension,
            hw_a
        ]
        .spacing(30)
        .into();

        let content = if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        };

        column![
            controls.padding([20, 50]),
            scrollable(container(content).padding(50)).height(Length::FillPortion(15)),
            container(row![horizontal_space(), convert_btn.padding([10, 40])])
                .height(Length::FillPortion(2))
                .padding([10, 20])
                .align_y(Alignment::Center)
        ]
        .into()
    }

    fn subscription(&self) -> Option<Subscription<Message>> {
        None
    }
}
impl SetupPage {
    pub fn new(options: ConverterOptions) -> SetupPage {
        let FfmpegOptions {
            hwaccel,
            audio_codec,
            video_codec,
            audio_bitrate,
            video_bitrate,
            output_extension,
            ..
        } = options.ffmpeg_options;

        let hw_accel = match hwaccel {
            Some(t) => t.into(),
            None => HwAccel::None,
        };

        let output_extension = match output_extension {
            lib_core::types::OutputExtension::Default => None,
            t => Some(t.into()),
        };

        Self {
            input_dir: Some(options.input_dir),
            output_dir: Some(options.output_dir),
            audio_bitrate: audio_bitrate.to_option(),
            video_bitrate: video_bitrate.to_option(),
            audio_codec: audio_codec.to_option().map(AudioCodec::from),
            video_codec: video_codec.to_option().map(VideoCodec::from),
            hw_accel,
            output_extension,
            ..Default::default()
        }
    }
    pub fn with_state(state: ProgressPage) -> SetupPage {
        SetupPage {
            converting_page_state: Some(state),
            ..Default::default()
        }
    }
    fn create_folder_selector<'a>(
        folder_for: FolderIden,
        path: Option<&PathBuf>,
        label: impl Into<String>,
    ) -> Column<'a, Message> {
        let path = if let Some(path) = path {
            path.to_str().unwrap_or("Invalid path").to_string()
        } else {
            "No folder selected".to_string()
        };

        column![
            text(label.into())
                .size(30)
                .font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                })
                .style(text::primary),
            container(
                scrollable(
                    rich_text![span(path).color(Color::BLACK).size(15).font(Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })]
                    .align_y(Alignment::Center),
                )
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::default()
                ))
                .height(Length::Fill)
                .width(Length::Fill)
            )
            .style(container::rounded_box)
            .width(Length::Fill)
            .height(60)
            .padding(10),
            row![
                button("Select Folder")
                    .on_press(Message::SetupPage(SetupPageMessage::SelectFolder(
                        folder_for
                    )))
                    .padding([10, 20])
            ],
        ]
        .spacing(20)
    }

    fn create_toggler_element<T>(
        data: Option<T>,
        label: impl Into<String>,
        inner: Element<Message>,
        toggle_switch: ToggleType,
    ) -> Element<Message> {
        let is_check = data.is_some();
        let ttext = if is_check { "Custom" } else { "Match Source" };

        let switch = container(row![
            toggler(is_check).on_toggle(move |b| {
                Message::SetupPage(SetupPageMessage::EnableToggle(toggle_switch, b))
            }),
            text(ttext)
        ])
        .align_y(Alignment::Center);

        let inner = container(if is_check {
            inner
        } else {
            row![horizontal_space()].into()
        })
        .width(Length::Fill)
        .height(Length::Fill);

        let content: Element<Message> = row![
            container(
                text(label.into())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center()
                    .font(Font {
                        weight: font::Weight::Bold,
                        ..Default::default()
                    })
            )
            .width(Length::FillPortion(2)),
            container(
                column![
                    switch.height(Length::FillPortion(3)),
                    inner.height(Length::FillPortion(3))
                ]
                .spacing(10)
            )
            .padding(10)
            .width(Length::FillPortion(3))
        ]
        .into();

        container(content)
            .center_y(Length::Fixed(135.))
            .style(container::bordered_box)
            .style(container::rounded_box)
            .into()
    }

    fn convert(
        &self,
        input_dir: PathBuf,
        output_dir: PathBuf,
    ) -> (
        Task<Message>,
        watch::Sender<bool>,
        Arc<RwLock<ProgressSystem>>,
        ConverterOptions,
    ) {
        let ac = self.audio_codec.unwrap_or_value(ArgsType::MatchSource);

        let vc = match self.video_codec {
            Some(vc) => match (self.hw_accel, vc) {
                (HwAccel::Cuda, VideoCodec::H264) => {
                    ArgsType::Custom(lib_core::types::VideoCodec::H264NVENC)
                }
                (HwAccel::Cuda, VideoCodec::H265) => {
                    ArgsType::Custom(lib_core::types::VideoCodec::H265NVENC)
                }
                (_, val) => val.into(),
            },
            None => ArgsType::MatchSource,
        };

        let ab = self.audio_bitrate.unwrap_or_value(ArgsType::MatchSource);

        let vb = self.video_bitrate.unwrap_or_value(ArgsType::MatchSource);

        let oex = self
            .output_extension
            .unwrap_or_value(lib_core::types::OutputExtension::Mkv);

        let ffmpeg_option = FfmpegOptions::new(
            // NOTE : Skipping resolution (not yet implemented)
            ArgsType::MatchSource,
            self.hw_accel.into(),
            ac,
            vc,
            ab,
            vb,
            // NOTE : Skipping picture format as it has not been implemented
            ArgsType::MatchSource,
            oex,
        );

        let (stop_tx, stop_rx) = watch::channel(false);

        let progress_system = Arc::new(RwLock::new(ProgressSystem::new(200)));

        let mut converter =
            Converter::new_with_progress_tracker(stop_rx.clone(), progress_system.clone());

        let options = ConverterOptions::new(input_dir, output_dir, true, ffmpeg_option);

        let converter_opts = options.clone();

        let task = Task::future(async move {
            converter.prepare_task(Arc::new(options)).await.unwrap();

            #[cfg(feature = "embedded")]
            converter
                .start_conversion(Some(ffmpeg_instance()))
                .await
                .unwrap();

            #[cfg(not(feature = "embedded"))]
            converter.start_conversion(None).await.unwrap();

            Message::ProgressPage(ProgressPageMessage::DoneConvert)
        });

        (task, stop_tx, progress_system, converter_opts)
    }
}

async fn select_folder() -> Option<PathBuf> {
    Some(
        rfd::AsyncFileDialog::new()
            .pick_folder()
            .await?
            .path()
            .to_path_buf(),
    )
}
