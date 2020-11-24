// Oliver Berzs
// https://github.com/oberzs/duku

use std::os::raw::c_void;

#[cfg(target_os = "linux")]
use std::os::raw::c_ulong;

/// Handle to a OS window.
///
/// Used to add the window to the
/// duku context.
#[cfg(target_os = "windows")]
#[derive(Debug, Copy, Clone)]
pub struct WindowHandle {
    /// Windows HWND handle
    pub hwnd: *const c_void,
}

/// Handle to a OS window.
///
/// Used to add the window to the
/// duku context.
#[cfg(target_os = "linux")]
#[derive(Debug, Copy, Clone)]
pub struct WindowHandle {
    /// Xlib window handle
    pub xlib_window: c_ulong,
    /// Xlib display handle
    pub xlib_display: *mut c_void,
}

/// Handle to a OS window.
///
/// Used to add the window to the
/// duku context.
#[cfg(target_os = "macos")]
#[derive(Debug, Copy, Clone)]
pub struct WindowHandle {
    /// MacOS window handle
    pub ns_window: *mut c_void,
}
