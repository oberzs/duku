mod device;
mod extensions;
mod instance;
mod validator;
mod window_surface;

use log::debug;
use log::info;

use device::Device;
use device::VSync;
use extensions::Extensions;
use instance::Instance;
use validator::Validator;
use window_surface::WindowArgs;
use window_surface::WindowSurface;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    _device: Device,
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

        #[cfg(debug_assertions)]
        debug!("create validator");
        #[cfg(debug_assertions)]
        let validator = Some(Validator::new(&instance));
        #[cfg(debug_assertions)]
        info!("validator created");
        #[cfg(not(debug_assertions))]
        let validator = None;

        debug!("create window surface");
        let window_surface = WindowSurface::new(&instance, args);
        info!("window surface created");

        debug!("open GPU");
        let device = Device::new(&instance, &window_surface, &extensions, VSync::Enabled, 0);
        info!("GPU opened");

        Self {
            _device: device,
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
