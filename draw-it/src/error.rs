// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// draw-it error types

use std::error::Error;
use std::ffi;
use std::ffi::CString;
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
    Json(serde_json::Error),
    Poison(sync::PoisonError<()>),
    VulkanInstance(ash::InstanceError),
    VulkanLoad(ash::LoadingError),
    VulkanCode(i32),
    #[cfg(feature = "image")]
    Image(image_file::ImageError),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    UnsupportedExtension(CString),
    UnsupportedValidation(CString),
    UnsupportedMemoryType,
    NoSuitableGpu,
    NoSuitableMemoryType,
}

impl Error for ErrorType {}

impl fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorType::VulkanCode(e) => write!(fmt, "vulkan code {}", e),
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

impl From<serde_json::Error> for ErrorType {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<sync::PoisonError<()>> for ErrorType {
    fn from(e: sync::PoisonError<()>) -> Self {
        Self::Poison(e)
    }
}

impl From<ash::InstanceError> for ErrorType {
    fn from(e: ash::InstanceError) -> Self {
        Self::VulkanInstance(e)
    }
}

impl From<ash::LoadingError> for ErrorType {
    fn from(e: ash::LoadingError) -> Self {
        Self::VulkanLoad(e)
    }
}

impl From<ash::vk::Result> for ErrorType {
    fn from(e: ash::vk::Result) -> Self {
        Self::VulkanCode(e.as_raw())
    }
}

#[cfg(feature = "image")]
impl From<image_file::ImageError> for ErrorType {
    fn from(e: image_file::ImageError) -> Self {
        Self::Image(e)
    }
}

impl From<ErrorKind> for ErrorType {
    fn from(e: ErrorKind) -> Self {
        Self::Internal(e)
    }
}
