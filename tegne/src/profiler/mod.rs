mod writer;

use std::path::Path;

#[cfg(feature = "profiler")]
pub(crate) use writer::ProfileTimer;
#[cfg(feature = "profiler")]
use writer::PROFILER;

pub fn begin_profile(_name: &'static str, _path: impl AsRef<Path>) {
    #[cfg(feature = "profiler")]
    PROFILER.lock().unwrap().begin(_name, _path).unwrap();
}

pub fn end_profile() {
    #[cfg(feature = "profiler")]
    PROFILER.lock().unwrap().end();
}

#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        #[cfg(feature = "profiler")]
        let _f =
            crate::profiler::ProfileTimer::new(concat!(module_path!(), "::", $name, ":", line!()));
    };
}
