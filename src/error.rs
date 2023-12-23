use std::{fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BufLengthMismatch,
    InvalidBitCount,
    InvalidCompression,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(why) => write!(f, "I/O Error: {why}"),
            Self::BufLengthMismatch => write!(f, "Buffer length is too short or long"),
            Self::InvalidBitCount => write!(f, "Invalid bit count value"),
            Self::InvalidCompression => write!(f, "Invalid compression value"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}
