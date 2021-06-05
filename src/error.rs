use std::{fmt, io};

#[derive(PartialEq)]
pub enum Error {
    GenericError(String),
    IoError(String),
    RecordNotFound(String),
    InvalidIP(String),
    InvalidState(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::GenericError(err.to_string())
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Error {
        Error::GenericError(err.to_string())
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Error {
        Error::GenericError(err.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Error {
        Error::GenericError(err.to_string())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GenericError(msg) => write!(f, "GenericError: {}", msg)?,
            Error::IoError(msg) => write!(f, "IoError: {}", msg)?,
            Error::RecordNotFound(msg) => write!(f, "RecordNotFound: {}", msg)?,
            Error::InvalidIP(msg) => write!(f, "InvalidIP: {}", msg)?,
            Error::InvalidState(msg) => write!(f, "InvalidState: {}", msg)?,
        }
        Ok(())
    }
}
