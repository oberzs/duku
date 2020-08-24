// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// draw-it error types

use std::error::Error;
use std::ffi;
use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::sync;

pub type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    // External error
    Io(io::Error),
    Nul(ffi::NulError),
    NoNul(ffi::FromBytesWithNulError),
    Binary(Box<bincode::ErrorKind>),
    Poison(sync::PoisonError<()>),
    #[cfg(feature = "window")]
    Window(glfw::InitError),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    UnsupportedExtension(String),
    UnsupportedMemoryType,
    NoSuitableGpu,
    NoSuitableMemoryType,
    #[cfg(feature = "ui")]
    UnitializedUi,
}

impl Error for ErrorType {}

impl fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ref e => write!(fmt, "{:?}", e),
        }
    }
}

impl From<io::Error> for ErrorType {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ffi::NulError> for ErrorType {
    fn from(e: ffi::NulError) -> Self {
        Self::Nul(e)
    }
}

impl From<ffi::FromBytesWithNulError> for ErrorType {
    fn from(e: ffi::FromBytesWithNulError) -> Self {
        Self::NoNul(e)
    }
}

impl From<Box<bincode::ErrorKind>> for ErrorType {
    fn from(err: Box<bincode::ErrorKind>) -> Self {
        Self::Binary(err)
    }
}

impl From<sync::PoisonError<()>> for ErrorType {
    fn from(e: sync::PoisonError<()>) -> Self {
        Self::Poison(e)
    }
}

#[cfg(feature = "window")]
impl From<glfw::InitError> for ErrorType {
    fn from(e: glfw::InitError) -> Self {
        Self::Window(e)
    }
}

impl From<ErrorKind> for ErrorType {
    fn from(e: ErrorKind) -> Self {
        Self::Internal(e)
    }
}
