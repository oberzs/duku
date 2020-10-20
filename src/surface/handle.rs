// Oliver Berzs
// https://github.com/oberzs/draw-it

// WindowHandle - OS window properties for surface creation

use std::os::raw::c_void;

#[cfg(target_os = "linux")]
use std::os::raw::c_ulong;

#[cfg(target_os = "windows")]
#[derive(Debug, Copy, Clone)]
pub struct WindowHandle {
    pub hwnd: *const c_void,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Copy, Clone)]
pub struct WindowHandle {
    pub xlib_window: c_ulong,
    pub xlib_display: *mut c_void,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Copy, Clone)]
pub struct WindowHandle {
    pub ns_window: *mut c_void,
}
