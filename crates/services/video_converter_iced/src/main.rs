#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use iced::{
    Element, Event, Size, Subscription, Task, Theme, event, exit, keyboard,
    keyboard::{key, key::Code},
    window,
};
use image::ImageFormat;

pub mod assets;
pub mod utils;

mod config;

mod pages;

use crate::config::Config;
use crate::pages::SetupPageMessage;
use crate::{
    assets::{Assets, ffmpeg_instance},
    pages::{Message, Page, Pages, ProgressPageMessage},
};

fn main() -> iced::Result {
    // console_subscriber::init();
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let log_name = format!("{}.log", timestamp);
    let mut log_dirs = directories::ProjectDirs::from("com", "re-converter", "app")
        .unwrap()
        .data_local_dir()
        .to_path_buf();

    log_dirs.push("logs");

    let appender = tracing_appender::rolling::never(log_dirs, log_name);
    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);

    tracing_subscriber::fmt()
        .compact()
        .with_ansi(false)
        .with_target(false)
        .with_thread_ids(true)
        .with_writer(non_blocking)
        .init();

    let win_setting = window::Settings {
        size: Size::new(500., 800.),
        resizable: false,
        icon: Some(
            window::icon::from_file_data(
                &Assets::get("icon.ico").unwrap().data,
                Some(ImageFormat::Ico),
            )
            .unwrap(),
        ),
        ..Default::default()
    };

    iced::application(ReConverter::title, ReConverter::update, ReConverter::view)
        .window(win_setting)
        .theme(ReConverter::theme)
        .subscription(ReConverter::subscription)
        .run_with(ReConverter::new)
}

#[derive(Debug, Clone, Default)]
struct ReConverter {
    page: Box<Pages>,
}

impl ReConverter {
    fn new() -> (ReConverter, Task<Message>) {
        tracing::info!("Starting application");
        (
            Self::default(),
            Task::future(async move {
                let op = { Config::get_instance().read().await }.last_saved.clone();

                Message::SetupPage(SetupPageMessage::DefaultValue(op))
            }),
        )
    }
    fn title(&self) -> String {
        "ReConverter".to_string()
    }
    fn update(&mut self, event: Message) -> Task<Message> {
        if let Message::EventOccurred(event) = &event {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    physical_key: key::Physical::Code(Code::KeyD),
                    ..
                }) => {
                    return Task::done(Message::ProgressPage(ProgressPageMessage::Debug));
                }
                Event::Window(window::Event::CloseRequested) => {
                    let _ = std::fs::remove_file(ffmpeg_instance());
                    return exit();
                }
                _ => {}
            }
        }
        let (task, page) = self.page.update(event);
        if let Some(page) = page {
            self.page = page;
        }

        task
    }

    fn view(&self) -> Element<Message> {
        self.page.view()
    }

    fn subscription(&self) -> Subscription<Message> {
        let event_listener = event::listen_with(|event, _, _| match event {
            Event::Keyboard(_) | Event::Window(_) => Some(Message::EventOccurred(event)),
            _ => None,
        });

        let mut subscriptions = vec![event_listener];

        if let Some(subs) = self.page.subscription() {
            subscriptions.push(subs);
        }

        Subscription::batch(subscriptions)
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
