use std::ffi::CString;
use std::process::exit;

pub fn cstring(s: &str) -> CString {
    CString::new(s).expect("cannot create CString")
}

pub fn error(s: &str) -> ! {
    log::error!("{}", s);
    exit(1);
}

pub fn unwrap_error<T, E>(result: Result<T, E>, s: &str) -> T {
    result.unwrap_or_else(|_| error(s))
}

pub fn unwrap_option<T>(option: Option<T>, s: &str) -> T {
    unwrap_error(option.ok_or(""), s)
}
