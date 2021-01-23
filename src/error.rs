// Oliver Berzs
// https://github.com/oberzs/duku

use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::io;

/// Crate's `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate's `Error` type.
#[derive(Debug)]
pub enum Error {
    /// Rust's io error
    Io(String),
    /// Invalid SPIR-V data
    InvalidSpirv,

    /// Unsupported PNG or JPEG color format
    #[cfg(any(feature = "png", feature = "jpeg"))]
    UnsupportedFormat,

    /// Invalid PNG data
    #[cfg(feature = "png")]
    InvalidPng,

    /// Invalid JPEG data
    #[cfg(feature = "jpeg")]
    InvalidJpeg,

    /// Invalid GLSL source
    #[cfg(feature = "glsl")]
    InvalidGlsl(String),

    /// Invalid GLTF data
    #[cfg(feature = "gltf")]
    InvalidGltf,
    /// Unsupported GLTF mime-type
    #[cfg(feature = "gltf")]
    UnsupportedMimeType(String),
    /// Unsupported GLTF primitive mode
    #[cfg(feature = "gltf")]
    UnsupportedPrimitive,

    /// Invalid OTF data
    #[cfg(feature = "otf")]
    InvalidOtf,
    /// Unsupported character
    #[cfg(feature = "otf")]
    UnsupportedChar(char),

    /// Non matching canvas
    #[cfg(feature = "gif")]
    NonMatchingCanvas,
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
