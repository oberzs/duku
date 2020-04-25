use log::debug;
use log::info;
use std::collections::HashMap;
use std::rc::Rc;

use super::Device;
use super::Extensions;
use super::Swapchain;
use super::Target;
use super::VSync;
use super::Validator;
use super::Vulkan;
use super::WindowArgs;
use super::WindowSurface;
use crate::builtins::BuiltinMaterial;
use crate::builtins::BuiltinMesh;
use crate::builtins::BuiltinShader;
use crate::builtins::BuiltinTexture;
use crate::builtins::Builtins;
use crate::images::Anisotropy;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::model::Material;
use crate::model::MaterialBuilder;
use crate::model::Mesh;
use crate::model::MeshBuilder;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    builtins: Builtins,
    window_framebuffers: Vec<Framebuffer>,
    render_passes: HashMap<RenderPassType, RenderPass>,
    image_uniforms: ImageUniforms,
    shader_layout: ShaderLayout,
    _swapchain: Swapchain,
    device: Rc<Device>,
    _window_surface: WindowSurface,
    _validator: Option<Validator>,
    _vulkan: Vulkan,
}

pub struct TegneBuilder {
    window_args: Option<WindowArgs>,
    anisotropy: Anisotropy,
    vsync: VSync,
}

#[derive(Hash, Eq, PartialEq)]
enum RenderPassType {
    // ColorOnscreen,
    ColorOffscreen,
    DepthOffscreen,
}

impl Tegne {
    pub fn builder() -> TegneBuilder {
        TegneBuilder {
            window_args: None,
            anisotropy: Anisotropy::Disabled,
            vsync: VSync::Disabled,
        }
    }

    pub fn draw_on_window(&self, draw_callback: impl Fn(&mut Target)) {
        let mut target = Target::new(&self.builtins);
        draw_callback(&mut target);
    }

    pub fn create_texture_rgba(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("create rgba texture");
        let texture =
            Texture::from_raw_rgba(&self.device, raw, width, height, &self.image_uniforms);
        info!("rgba texture created");
        texture
    }

    pub fn create_texture_rgb(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("create rgb texture");
        let texture = Texture::from_raw_rgb(&self.device, raw, width, height, &self.image_uniforms);
        info!("rgb texture created");
        texture
    }

    pub fn create_mesh(&self) -> MeshBuilder {
        Mesh::builder(&self.device)
    }

    pub fn create_material(&self) -> MaterialBuilder {
        let default_shader = self.builtins.get_shader(BuiltinShader::Phong);
        let default_texture = self.builtins.get_texture(BuiltinTexture::White);
        Material::builder(
            &self.device,
            default_shader,
            default_texture,
            &self.shader_layout,
        )
    }

    pub fn create_framebuffer(&self, width: u32, height: u32) -> Framebuffer {
        let render_pass = self
            .render_passes
            .get(&RenderPassType::ColorOffscreen)
            .or_error("render passes not setup");
        Framebuffer::new(
            &self.device,
            render_pass,
            &self.image_uniforms,
            &self.shader_layout,
            width,
            height,
        )
    }
}

impl TegneBuilder {
    pub fn build(&self) -> Tegne {
        let window_args = self.window_args.or_error("window arguments not set");
        let extensions = Extensions::new();

        debug!("create Vulkan instance");
        let vulkan = Vulkan::new(&extensions);
        info!("Vulkan instance created");

        #[cfg(debug_assertions)]
        debug!("create validator");
        #[cfg(debug_assertions)]
        let validator = Some(Validator::new(&vulkan));
        #[cfg(debug_assertions)]
        info!("validator created");
        #[cfg(not(debug_assertions))]
        let validator = None;

        debug!("create window surface");
        let window_surface = WindowSurface::new(&vulkan, window_args);
        info!("window surface created");

        debug!("open GPU");
        let device = Device::new(&vulkan, &window_surface, &extensions, self.vsync, 1);
        info!("GPU opened");

        debug!("create window swapchain");
        let swapchain = Swapchain::new(
            &vulkan,
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

        debug!("create render passes");
        let c_render_pass = RenderPass::color_offscreen(&device);
        let d_render_pass = RenderPass::depth_offscreen(&device);
        info!("render passes created");

        let builtins = Builtins::new(&device, &c_render_pass, &shader_layout, &image_uniforms);

        debug!("create window framebuffers");
        let window_framebuffers = Framebuffer::for_window(
            &device,
            &swapchain,
            &c_render_pass,
            &image_uniforms,
            &shader_layout,
            window_args.width,
            window_args.height,
        );
        info!("window framebuffers created");

        let mut render_passes = HashMap::new();
        render_passes.insert(RenderPassType::ColorOffscreen, c_render_pass);
        render_passes.insert(RenderPassType::DepthOffscreen, d_render_pass);

        Tegne {
            builtins,
            window_framebuffers,
            render_passes,
            image_uniforms,
            shader_layout,
            _swapchain: swapchain,
            device,
            _window_surface: window_surface,
            _validator: validator,
            _vulkan: vulkan,
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
