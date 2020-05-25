// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// tegne error types

use crossbeam::channel;
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
    Signal(channel::SendError<()>),
    Poison(sync::PoisonError<()>),
    Png(png::ImageError),
    VulkanInstance(ash::InstanceError),
    VulkanLoad(ash::LoadingError),
    VulkanCode(i32),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    UnsupportedExtension(CString),
    UnsupportedValidation(CString),
    UnsupportedMemoryType,
    UnsupportedMsaa,
    NoSuitableGpu,
    NoSuitableMemoryType,
    InvalidMsaa,
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

impl From<channel::SendError<()>> for ErrorType {
    fn from(e: channel::SendError<()>) -> Self {
        Self::Signal(e)
    }
}

impl From<png::ImageError> for ErrorType {
    fn from(e: png::ImageError) -> Self {
        Self::Png(e)
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

impl From<ErrorKind> for ErrorType {
    fn from(e: ErrorKind) -> Self {
        Self::Internal(e)
    }
}
