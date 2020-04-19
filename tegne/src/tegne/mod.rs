mod device;
mod extensions;
mod instance;
mod swapchain;
mod validator;
mod window_surface;

use log::debug;
use log::info;
use std::rc::Rc;

use crate::shaders::ImageUniforms;
use crate::shaders::ShaderLayout;
pub use device::Device;
use device::VSync;
use extensions::Extensions;
use instance::Instance;
pub use swapchain::Swapchain;
use validator::Validator;
use window_surface::WindowArgs;
use window_surface::WindowSurface;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    _image_uniforms: ImageUniforms,
    _shader_layout: ShaderLayout,
    _swapchain: Swapchain,
    _device: Rc<Device>,
    _window_surface: WindowSurface,
    _validator: Option<Validator>,
    _instance: Instance,
}

impl Tegne {
    pub fn new(args: WindowArgs, anisotropy: u8) -> Self {
        let width = args.width;
        let height = args.height;
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
        let device = Rc::new(Device::new(
            &instance,
            &window_surface,
            &extensions,
            VSync::Enabled,
            0,
        ));
        info!("GPU opened");

        debug!("create window swapchain");
        let swapchain = Swapchain::new(&instance, &device, &window_surface, width, height);
        info!("window swapchain created");

        debug!("create shader layout");
        let shader_layout = ShaderLayout::new(&device);
        info!("shader layout created");

        debug!("create image uniforms");
        let image_uniforms = ImageUniforms::new(&device, &shader_layout, anisotropy);
        info!("image uniforms created");

        Self {
            _image_uniforms: image_uniforms,
            _shader_layout: shader_layout,
            _swapchain: swapchain,
            _device: device,
            _window_surface: window_surface,
            _validator: validator,
            _instance: instance,
        }
    }

    #[cfg(feature = "tegne-utils")]
    pub fn from_window(window: &Window, anisotropy: u8) -> Self {
        #[cfg(target_os = "windows")]
        let args = WindowArgs {
            hwnd: window.hwnd(),
            width: window.width(),
            height: window.height(),
        };

        #[cfg(target_os = "linux")]
        let args = WindowArgs {
            xlib_window: window.xlib_window(),
            xlib_display: window.xlib_display(),
            width: window.width(),
            height: window.height(),
        };

        #[cfg(target_os = "macos")]
        let args = WindowArgs {
            ns_window: window.ns_window(),
            ns_view: window.ns_view(),
            width: window.width(),
            height: window.height(),
        };

        Self::new(args, anisotropy)
    }
}
