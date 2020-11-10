// Oliver Berzs
// https://github.com/oberzs/duku

// duku error types

use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(String),
    InvalidSpirv,
    InvalidFile,
    NoSuitableGpu,

    #[cfg(feature = "png")]
    UnsupportedColorType,
    InvalidPng,

    #[cfg(feature = "glsl")]
    InvalidGlsl(String),

    #[cfg(feature = "window")]
    InternalGlfw,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "glsl")]
        match self {
            Self::InvalidGlsl(s) => write!(fmt, "{}", s),
            e => write!(fmt, "{:?}", e),
        }
        #[cfg(not(feature = "glsl"))]
        write!(fmt, "{:?}", self)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(format!("{}", e))
    }
}
