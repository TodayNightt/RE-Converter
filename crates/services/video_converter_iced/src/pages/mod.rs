use iced::{Element, Event, Subscription, Task};
use std::fmt::Debug;

mod progress;
mod setup;

use crate::pages::progress::ProgressPage;
pub use crate::pages::setup::SetupPage;
pub use progress::ProgressPageMessage;
pub(crate) use setup::SetupPageMessage;

#[derive(Debug, Clone)]
pub enum Pages {
    Setup(SetupPage),
    Progress(ProgressPage),
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    EventOccurred(Event),
    SetupPage(SetupPageMessage),
    ProgressPage(ProgressPageMessage),
}

impl Default for Pages {
    fn default() -> Self {
        Pages::Setup(SetupPage::default())
    }
}

impl Page for Pages {
    fn update(&mut self, message: Message) -> (Task<Message>, Option<Box<Pages>>) {
        match self {
            Pages::Setup(page) => page.update(message),
            Pages::Progress(page) => page.update(message),
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            Pages::Setup(page) => page.view(),
            Pages::Progress(page) => page.view(),
        }
    }

    fn subscription(&self) -> Option<Subscription<Message>> {
        match self {
            Pages::Setup(page) => page.subscription(),
            Pages::Progress(page) => page.subscription(),
        }
    }
}

pub trait Page: Debug + Clone + Default {
    fn update(&mut self, message: Message) -> (Task<Message>, Option<Box<Pages>>);
    fn view(&self) -> Element<Message>;
    fn subscription(&self) -> Option<Subscription<Message>>;
}
