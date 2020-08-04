// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// draw-it-import error types

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io;

pub type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    // External error
    Io(io::Error),
    Binary(Box<bincode::ErrorKind>),
    Shader(shaderc::Error),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    InvalidFont,
    InvalidShader(String),
    NoBounds,
    NoCompiler,
}

impl Error for ErrorType {}

impl fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorType::Internal(ErrorKind::InvalidShader(ref err)) => write!(fmt, "{}", err),
            ref e => write!(fmt, "{:?}", e),
        }
    }
}

impl From<io::Error> for ErrorType {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<Box<bincode::ErrorKind>> for ErrorType {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Self::Binary(err)
    }
}

impl From<shaderc::Error> for ErrorType {
    fn from(err: shaderc::Error) -> Self {
        Self::Shader(err)
    }
}

impl From<ErrorKind> for ErrorType {
    fn from(e: ErrorKind) -> Self {
        Self::Internal(e)
    }
}
