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

    #[cfg(any(feature = "png", feature = "jpeg"))]
    UnsupportedFormat,

    #[cfg(feature = "png")]
    InvalidPng,

    #[cfg(feature = "jpeg")]
    InvalidJpeg,

    #[cfg(feature = "glsl")]
    InvalidGlsl(String),

    #[cfg(feature = "window")]
    InternalGlfw,

    #[cfg(feature = "gltf")]
    InvalidGltf,
    #[cfg(feature = "gltf")]
    UnsupportedMimeType(String),
    #[cfg(feature = "gltf")]
    UnsupportedPrimitive,
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
