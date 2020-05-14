use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::io;

pub type Result<T> = std::result::Result<T, ErrorType>;

#[derive(Debug)]
pub enum ErrorType {
    // External error
    Io(io::Error),
    VulkanInstance(ash::InstanceError),
    VulkanLoad(ash::LoadingError),
    VulkanCode(i32),
    // Internal error
    Internal(ErrorKind),
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorKind {
    DeviceDropped,
    UnsupportedExtension,
    UnsupportedMsaa,
    NoSuitableGpu,
    NoSuitableMemoryType,
}

impl Error for ErrorType {}

impl fmt::Display for ErrorType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorType::Io(ref e) => write!(fmt, "{:?}", e),
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
