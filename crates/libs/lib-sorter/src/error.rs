use std::{fmt::Display, io};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    WrongDatetime,
    ToOsStringError,
    CreateFileError(String),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongDatetime => f.write_str("wrong datetime"),
            _ => f.write_str("io"),
        }
    }
}

impl std::error::Error for Error {}
