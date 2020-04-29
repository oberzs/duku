use log::error;
use std::process::exit;

pub trait OrError<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T;
}

impl<T, E> OrError<T> for Result<T, E> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|_| {
            error!("{}", msg.as_ref());
            exit(1);
        })
    }
}

impl<T> OrError<T> for Option<T> {
    fn or_error(self, msg: impl AsRef<str>) -> T {
        self.unwrap_or_else(|| {
            error!("{}", msg.as_ref());
            exit(1);
        })
    }
}
