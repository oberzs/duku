use std::error::Error;
use std::ffi;
use std::fmt;
use std::fmt::Formatter;
use std::io;
use std::sync::mpsc;

pub type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    // External error
    Io(io::Error),
    Nul(ffi::NulError),
    Json(serde_json::Error),
    Signal(mpsc::SendError<()>),
    Image(image::ImageError),
    VulkanInstance(ash::InstanceError),
    VulkanLoad(ash::LoadingError),
    VulkanCode(i32),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    UnsupportedExtension,
    UnsupportedMsaa,
    NoSuitableGpu,
    NoSuitableMemoryType,
    InvalidMsaa,
    NoVertices,
    NoTriangles,
    TooManyUvs,
    TooManyNormals,
}

impl Error for ErrorType {}

impl fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorType::Io(ref e) => write!(fmt, "{:?}", e),
            ErrorType::Nul(ref e) => write!(fmt, "{:?}", e),
            ErrorType::Json(ref e) => write!(fmt, "{:?}", e),
            ErrorType::Signal(ref e) => write!(fmt, "{:?}", e),
            ErrorType::Image(ref e) => write!(fmt, "{:?}", e),
            ErrorType::VulkanInstance(ref e) => write!(fmt, "{:?}", e),
            ErrorType::VulkanLoad(ref e) => write!(fmt, "{:?}", e),
            ErrorType::VulkanCode(e) => write!(fmt, "vulkan code {:?}", e),
            ErrorType::Internal(ref e) => write!(fmt, "{:?}", e),
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

impl From<serde_json::Error> for ErrorType {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}

impl From<mpsc::SendError<()>> for ErrorType {
    fn from(e: mpsc::SendError<()>) -> Self {
        Self::Signal(e)
    }
}

impl From<image::ImageError> for ErrorType {
    fn from(e: image::ImageError) -> Self {
        Self::Image(e)
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
