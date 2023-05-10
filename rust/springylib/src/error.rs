use crate::media::txt::DecryptError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    UnknownFormat(String),
    InvalidExtension(Option<String>),
    InvalidPath(String),
    InvalidData {
        info: Option<String>,
        context: String,
    },
    Custom(Box<dyn std::error::Error>),
    UnknownError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownFormat(format) => write!(f, "Unknown format: {}", format),
            Error::UnknownError => write!(f, "Unknown Error"),
            Error::InvalidExtension(None) => write!(f, "Missing file extension"),
            Error::InvalidExtension(Some(ext)) => write!(f, "Invalid extension {}", ext),
            Error::InvalidPath(path) => write!(f, "Invalid Path {}", path),
            Error::InvalidData { info, context } => write!(
                f,
                "Invalid data: {}; {}",
                info.clone().unwrap_or("[no info]".to_string()),
                context
            ),
            Error::Custom(error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<binrw::Error> for Error {
    fn from(value: binrw::Error) -> Self {
        Error::Custom(Box::new(value))
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Custom(Box::new(value))
    }
}

impl From<serde_xml_rs::Error> for Error {
    fn from(value: serde_xml_rs::Error) -> Self {
        Error::Custom(Box::new(value))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Error::Custom(Box::new(value))
    }
}

impl From<DecryptError> for Error {
    fn from(value: DecryptError) -> Self {
        Error::Custom(Box::new(value))
    }
}
