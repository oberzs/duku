use std::ffi::CString;
use std::process::exit;

pub(crate) trait OrError<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T;
}

impl<T, E> OrError<T> for Result<T, E> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|_| error(msg))
    }
}

impl<T> OrError<T> for Option<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|| error(msg))
    }
}

pub(crate) fn cstring(s: impl AsRef<str>) -> CString {
    CString::new(s.as_ref()).or_error("cannot create CString")
}

pub(crate) fn error(msg: impl AsRef<str>) -> ! {
    log::error!("{}", msg.as_ref());
    exit(1);
}

pub(crate) fn clamp(n: u32, min: u32, max: u32) -> u32 {
    if n > max {
        max
    } else if n < min {
        min
    } else {
        n
    }
}
