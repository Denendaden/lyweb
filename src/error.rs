use std::{error::Error, fmt};

#[derive(Debug)]
pub enum LyError {
    NotFound,
    InternalServerError,
}

impl LyError {
    pub fn http_code(&self) -> u16 {
        match *self {
            Self::NotFound => 404,
            Self::InternalServerError => 500,
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
