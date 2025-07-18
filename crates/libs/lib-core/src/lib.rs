mod converter;
mod copiee;
mod exec;
mod progress;
pub mod types;

pub use error::{Error, Result};
pub use progress::{Progress, ProgressMonitor, ProgressSystem, Stage};

mod error {
    use crate::progress;
    use std::fmt::{Debug, Display};

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        NotExistanceInputOutputDir,
        CouldNotCreateDir(String),
        CopyError(String),
        ReadDirError(String),
        FfmpegError(String),
        ConverterHasNoTaskAvailable,
        SinkerError(String),
        ProgressTrackerError(progress::Error),
    }

    impl From<lib_sorter::Error> for Error {
        fn from(value: lib_sorter::Error) -> Self {
            Error::SinkerError(value.to_string())
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let _ = f.align();
            match self {
                Error::NotExistanceInputOutputDir => f.write_str("Input or Output dir are invalid"),
                Error::CopyError(s) => f.write_str(s),
                Error::CouldNotCreateDir(s) => f.write_str(s),
                Error::FfmpegError(ff) => f.write_str(ff),
                Error::ReadDirError(r) => f.write_str(r),
                Error::ConverterHasNoTaskAvailable => f.write_str("Internal Error"),
                Error::SinkerError(s) => f.write_str(s),
                Error::ProgressTrackerError(e) => std::fmt::Display::fmt(e, f),
            }
        }
    }

    impl std::error::Error for Error {}

    impl From<progress::Error> for Error {
        fn from(value: progress::Error) -> Self {
            Error::ProgressTrackerError(value)
        }
    }
}
