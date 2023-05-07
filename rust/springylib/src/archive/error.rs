use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BinRw(binrw::Error),
    Io(std::io::Error),
    Unsupported { reason: String },
}

impl From<binrw::Error> for Error {
    fn from(value: binrw::Error) -> Self {
        Error::BinRw(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BinRw(e) => write!(f, "{}", e),
            Error::Io(e) => write!(f, "{}", e),
            Error::Unsupported { reason } => write!(f, "Unsupported Archive: {}", reason),
        }
    }
}
