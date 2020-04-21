mod device;
mod extensions;
mod instance;
mod swapchain;
mod validator;
mod window_surface;

use log::debug;
use log::info;
use std::rc::Rc;

pub use crate::images::Anisotropy;
pub use crate::images::Texture;
use crate::images::TextureFormat;
use crate::model::Material;
pub use crate::model::MaterialBuilder;
use crate::model::Mesh;
pub use crate::model::MeshBuilder;
use crate::shaders::ImageUniforms;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;
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
    image_uniforms: ImageUniforms,
    shader_layout: ShaderLayout,
    _swapchain: Swapchain,
    device: Rc<Device>,
    _window_surface: WindowSurface,
    _validator: Option<Validator>,
    _instance: Instance,
}

pub struct TegneBuilder {
    window_args: Option<WindowArgs>,
    anisotropy: Anisotropy,
    vsync: VSync,
}

impl Tegne {
    pub fn builder() -> TegneBuilder {
        TegneBuilder {
            window_args: None,
            anisotropy: Anisotropy::Disabled,
            vsync: VSync::Disabled,
        }
    }

    pub fn create_texture_from_rgba(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("create rgba texture");
        let texture = Texture::from_raw(
            &self.device,
            raw,
            width,
            height,
            TextureFormat::RGBA,
            &self.image_uniforms,
        );
        info!("rgba texture created");
        texture
    }

    pub fn create_texture_from_rgb(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("create rgb texture");
        let texture = Texture::from_raw(
            &self.device,
            raw,
            width,
            height,
            TextureFormat::RGB,
            &self.image_uniforms,
        );
        info!("rgb texture created");
        texture
    }

    pub fn create_material(&self) -> MaterialBuilder {
        Material::builder(&self.device, &self.shader_layout)
    }

    pub fn create_mesh(&self) -> MeshBuilder {
        Mesh::builder(&self.device)
    }
}

impl TegneBuilder {
    pub fn build(&self) -> Tegne {
        let window_args = self.window_args.or_error("window arguments not set");
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
        let window_surface = WindowSurface::new(&instance, window_args);
        info!("window surface created");

        debug!("open GPU");
        let device = Device::new(&instance, &window_surface, &extensions, self.vsync, 0);
        info!("GPU opened");

        debug!("create window swapchain");
        let swapchain = Swapchain::new(
            &instance,
            &device,
            &window_surface,
            window_args.width,
            window_args.height,
        );
        info!("window swapchain created");

        debug!("create shader layout");
        let shader_layout = ShaderLayout::new(&device);
        info!("shader layout created");

        debug!("create image uniforms");
        let image_uniforms = ImageUniforms::new(&device, &shader_layout, self.anisotropy);
        info!("image uniforms created");

        Tegne {
            image_uniforms: image_uniforms,
            shader_layout,
            _swapchain: swapchain,
            device,
            _window_surface: window_surface,
            _validator: validator,
            _instance: instance,
        }
    }

    #[cfg(feature = "tegne-utils")]
    pub fn with_window(&mut self, window: &Window) -> &mut Self {
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

        self.window_args = Some(args);
        self
    }

    pub fn with_window_args(&mut self, value: WindowArgs) -> &mut Self {
        self.window_args = Some(value);
        self
    }

    pub fn with_anisotropy(&mut self, value: Anisotropy) -> &mut Self {
        self.anisotropy = value;
        self
    }

    pub fn with_vsync(&mut self) -> &mut Self {
        self.vsync = VSync::Enabled;
        self
    }
}
