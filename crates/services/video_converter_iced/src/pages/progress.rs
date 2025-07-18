use crate::Message;
use crate::assets::svg::back_arrow;
use crate::pages::setup::SetupPage;
use crate::pages::{Page, Pages};
use iced::time::every;

use iced::widget::{
    button, column, container, horizontal_space, progress_bar, rich_text, row, scrollable, span,
    text,
};
use iced::{Alignment, Color, Element, Font, Length, Subscription, Task, font};
use lib_core::{Progress, ProgressSystem};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, watch};
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub enum ProgressPageMessage {
    ChangeToSetupPage,
    UpdateProgress(Option<Arc<[Progress]>>),
    StopConvert,
    DoneConvert,
    Tick,
    Debug,
}

#[derive(Debug, Clone, Default)]
pub struct ProgressPage {
    progress: Arc<[Progress]>,
    stop_signal: watch::Sender<bool>,
    progress_system: Option<Arc<RwLock<ProgressSystem>>>,
    debug: bool,
    converting: bool,
}

impl Page for ProgressPage {
    fn update(&mut self, message: Message) -> (Task<Message>, Option<Box<Pages>>) {
        if let Message::ProgressPage(message) = message {
            match message {
                ProgressPageMessage::ChangeToSetupPage => {
                    if self.converting {
                        return (
                            Task::none(),
                            Some(Box::new(Pages::Setup(SetupPage::with_state(self.clone())))),
                        );
                    }
                    return (
                        Task::none(),
                        Some(Box::new(Pages::Setup(SetupPage::default()))),
                    );
                }

                ProgressPageMessage::UpdateProgress(progress) => {
                    self.converting = true;
                    if let Some(progress) = progress {
                        self.progress = progress;
                    }
                }
                ProgressPageMessage::StopConvert => {
                    if !self.stop_signal.is_closed() {
                        self.converting = false;
                        self.stop_signal.send(true).unwrap();
                        self.progress = Arc::new([]);
                    }
                }

                ProgressPageMessage::DoneConvert => {
                    if !self.progress.iter().all(|p| p.done()) {
                        return (
                            Task::future(async {
                                sleep(Duration::from_millis(300)).await;
                                Message::ProgressPage(ProgressPageMessage::DoneConvert)
                            }),
                            None,
                        );
                    }

                    self.converting = false;

                    self.progress_system = None;
                }
                ProgressPageMessage::Debug => self.debug = !self.debug,
                ProgressPageMessage::Tick => {
                    let ps = self.progress_system.clone();
                    return (
                        Task::future(async move {
                            // Note progress_system is already a Some Type when Tick is called
                            let ps = ps.unwrap();

                            Message::ProgressPage(ProgressPageMessage::UpdateProgress(
                                { ps.write().await }.get_progress().await,
                            ))
                        }),
                        None,
                    );
                }
            };
        }
        (Task::none(), None)
    }
    fn view(&self) -> Element<Message> {
        let back_btn = button(
            row![back_arrow(), "Back"]
                .spacing(10)
                .padding(5)
                .align_y(Alignment::Center),
        )
        .on_press(Message::ProgressPage(
            ProgressPageMessage::ChangeToSetupPage,
        ))
        .style(button::text);

        let pb = scrollable(if !self.progress.is_empty() {
            let mp = column(
                self.progress
                    .iter()
                    .map(|p| {
                        container(
                            column![
                                row![
                                    rich_text![span(format!("[{}]", p.folder())).size(16).font(
                                        Font {
                                            weight: font::Weight::Bold,
                                            ..Default::default()
                                        }
                                    )]
                                    .width(Length::Fixed(80.)),
                                    horizontal_space().width(Length::Fill),
                                    text(p.file().to_string())
                                        .width(Length::Shrink)
                                        .size(16)
                                        .font(Font {
                                            weight: font::Weight::Bold,
                                            ..Default::default()
                                        })
                                ]
                                .align_y(Alignment::Center),
                                row![
                                    progress_bar(0f32..=(p.total() as f32), p.count() as f32)
                                        .width(Length::FillPortion(8)),
                                    text(format!("{:2}/{:2}", p.count(), p.total()))
                                        .font(Font {
                                            weight: font::Weight::Bold,
                                            ..Default::default()
                                        })
                                        .width(Length::Fixed(48.))
                                        .center()
                                ]
                                .align_y(Alignment::Center)
                            ]
                            .spacing(10),
                        )
                        .style(container::bordered_box)
                        .style(container::rounded_box)
                        .padding(20)
                    })
                    .map(Element::from),
            )
            .spacing(20);

            container(mp).padding(10)
        } else {
            container(horizontal_space())
        });

        let cancel_btn = row![
            button("Cancel").on_press(Message::ProgressPage(ProgressPageMessage::StopConvert))
        ];

        let content: Element<Message> = column![
            back_btn.height(Length::Fixed(50.)),
            pb.height(Length::Fill),
            cancel_btn.height(Length::Fixed(50.))
        ]
        .spacing(30)
        .into();

        container(if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        })
        .padding(40)
        .into()
    }

    fn subscription(&self) -> Option<Subscription<Message>> {
        if self.progress_system.is_some() {
            return Some(
                every(Duration::from_millis(500))
                    .map(|_| Message::ProgressPage(ProgressPageMessage::Tick)),
            );
        }

        None
    }
}

impl ProgressPage {
    pub fn new(
        stop_signal: watch::Sender<bool>,
        progress_system: Arc<RwLock<ProgressSystem>>,
    ) -> Self {
        Self {
            stop_signal,
            progress_system: Some(progress_system),
            ..Default::default()
        }
    }
}
