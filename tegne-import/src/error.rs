use image::error::ImageError;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io;

pub type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    // External error
    Io(io::Error),
    Image(ImageError),
    Json(serde_json::Error),
    Shader(shaderc::Error),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    InvalidFont,
    NoBounds,
    NoCompiler,
}

impl Error for ErrorType {}

impl fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorType::Io(ref err) => write!(fmt, "{:?}", err),
            ErrorType::Image(ref err) => write!(fmt, "{:?}", err),
            ErrorType::Json(ref err) => write!(fmt, "{:?}", err),
            ErrorType::Shader(ref err) => write!(fmt, "{:?}", err),
            ErrorType::Internal(ref err) => write!(fmt, "{:?}", err),
        }
    }
}

impl From<io::Error> for ErrorType {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<ImageError> for ErrorType {
    fn from(err: ImageError) -> Self {
        Self::Image(err)
    }
}

impl From<serde_json::Error> for ErrorType {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<shaderc::Error> for ErrorType {
    fn from(err: shaderc::Error) -> Self {
        Self::Shader(err)
    }
}
