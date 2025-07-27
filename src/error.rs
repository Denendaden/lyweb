use std::{error::Error, fmt};

/// Provides error handling.
#[derive(Debug)]
pub enum LyError {
    /// A page or resource was not found (HTTP 404).
    NotFound,
    /// Something went wrong internally with the server (HTTP 500).
    /// TODO: split into more finely-tuned errors.
    InternalServerError,
    /// Something went wrong with compiling a webpage (HTTP 500).
    TemplatingError,
}

impl LyError {
    /// Returns the HTTP response status code associated with the error.
    pub fn http_code(&self) -> u16 {
        match *self {
            Self::NotFound => 404,
            Self::InternalServerError => 500,
            // default to error code 500 (Internal Server Error)
            _ => 500,
        }
    }
}

impl fmt::Display for LyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.http_code(), self)
    }
}

impl Error for LyError {}

impl From<std::io::Error> for LyError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound,
            _ => Self::InternalServerError,
        }
    }
}

impl From<regex::Error> for LyError {
    fn from(_err: regex::Error) -> Self {
        Self::TemplatingError
    }
}
