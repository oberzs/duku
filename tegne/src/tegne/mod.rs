mod extensions;
mod instance;
mod validator;
mod window_surface;

use log::debug;
use log::info;

use crate::utils::error;
use extensions::Extensions;
use instance::Instance;
use validator::Validator;
use window_surface::WindowArgs;
use window_surface::WindowSurface;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    _window_surface: WindowSurface,
    _validator: Option<Validator>,
    _instance: Instance,
}

impl Tegne {
    pub fn new(args: WindowArgs) -> Self {
        let extensions = Extensions::new();

        debug!("create Vulkan instance");
        let instance = Instance::new(&extensions);
        info!("Vulkan instance created");

        let validator = if cfg!(debug_assertions) {
            debug!("create validator");
            let val = Validator::new(&instance);
            info!("validator created");
            Some(val)
        } else {
            None
        };

        debug!("create window surface");
        let window_surface = WindowSurface::new(&instance, args);
        info!("window surface created");

        Self {
            _window_surface: window_surface,
            _validator: validator,
            _instance: instance,
        }
    }

    #[cfg(feature = "tegne-utils")]
    pub fn from_window(window: &Window) -> Self {
        #[cfg(target_os = "windows")]
        let args = WindowArgs {
            hwnd: window.hwnd(),
        };

        #[cfg(target_os = "linux")]
        let args = WindowArgs {
            xlib_window: window.xlib_window(),
            xlib_display: window.xlib_display(),
        };

        #[cfg(target_os = "macos")]
        let args = WindowArgs {
            ns_window: window.ns_window(),
            ns_view: window.ns_view(),
        };

        Self::new(args)
    }
}
