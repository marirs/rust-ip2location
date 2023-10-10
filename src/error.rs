use std::{fmt, io};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    GenericError(String),
    IoError(String),
    RecordNotFound,
    UnknownDb,
    InvalidBinDatabase(u8, u8),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err.to_string())
    }
}

// Use default implementation for `std::error::Error`
impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::GenericError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GenericError(msg) => write!(f, "GenericError: {}", msg)?,
            Error::IoError(msg) => write!(f, "IoError: {}", msg)?,
            Error::RecordNotFound => write!(f, "RecordNotFound: no record found")?,
            Error::UnknownDb => write!(
                f,
                "Unknown database: Database type should be Proxy or Location"
            )?,
            Error::InvalidBinDatabase(y, p) => write!(f, "Invalid Bin Database: {} {}", y, p)?,
        }
        Ok(())
    }
}
