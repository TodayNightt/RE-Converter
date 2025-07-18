mod monitor;
mod system;
mod tracker;
mod types;

pub use {
    monitor::ProgressMonitor,
    system::ProgressSystem,
    tracker::Stage,
    types::{JobInfo, Message, Progress},
};

pub use error::{Error, Result};

mod error {
    use std::fmt::{Display, Formatter};
    use std::sync::Arc;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, Clone)]
    pub enum Error {
        UpdateSignalFailed(String, Arc<str>),
        CreateSignalFailed(Arc<str>),
        DoneSignalFailed(Arc<str>),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::UpdateSignalFailed(file, folder) => {
                    write!(f, "Update signal failed at {}[{}]", file, folder)
                }
                Error::CreateSignalFailed(ctx) => write!(f, "Create signal failed at {}", ctx),
                Error::DoneSignalFailed(ctx) => write!(f, "Done signal failed at {}", ctx),
            }
        }
    }

    impl core::error::Error for Error {}
}
