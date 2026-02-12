//! Error types returned by database operations.

use std::{fmt, io};

/// Errors that can occur when opening a database or performing a lookup.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Catch-all for parse failures, slice errors, etc.
    GenericError(String),
    /// File system errors (missing file, permission denied, …).
    IoError(String),
    /// The IP address was not found in the database.
    RecordNotFound,
    /// The file does not appear to be a valid IP2Location or IP2Proxy BIN database.
    UnknownDb,
    /// The BIN header contains an unsupported `(db_year, product_code)` combination.
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
