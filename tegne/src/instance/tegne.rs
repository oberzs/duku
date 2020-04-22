use log::debug;
use log::info;
use std::collections::HashMap;
use std::rc::Rc;

use super::Device;
use super::Extensions;
use super::Swapchain;
use super::VSync;
use super::Validator;
use super::Vulkan;
use super::WindowArgs;
use super::WindowSurface;
use crate::images::Anisotropy;
use crate::images::Texture;
use crate::images::TextureFormat;
use crate::model::builtin;
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
    builtin_shaders: HashMap<BuiltinShader, Shader>,
    builtin_textures: HashMap<BuiltinTexture, Texture>,
    builtin_meshes: HashMap<BuiltinMesh, Mesh>,
    builtin_materials: HashMap<BuiltinMaterial, Material>,
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
    ColorOnscreen,
    ColorOffscreen,
    DepthOffscreen,
}

#[derive(Hash, Eq, PartialEq)]
enum BuiltinShader {
    Passthru,
    Texture,
    Unshaded,
}

#[derive(Hash, Eq, PartialEq)]
enum BuiltinTexture {
    White,
}

#[derive(Hash, Eq, PartialEq)]
enum BuiltinMesh {
    Cube,
    Sphere,
}

#[derive(Hash, Eq, PartialEq)]
enum BuiltinMaterial {
    Wireframe,
    Shadow,
    White,
}

impl Tegne {
    pub fn builder() -> TegneBuilder {
        TegneBuilder {
            window_args: None,
            anisotropy: Anisotropy::Disabled,
            vsync: VSync::Disabled,
        }
    }

    pub fn create_texture_rgba(&self, raw: &[u8], width: u32, height: u32) -> Texture {
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

    pub fn create_texture_rgb(&self, raw: &[u8], width: u32, height: u32) -> Texture {
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

    pub fn create_mesh(&self) -> MeshBuilder {
        Mesh::builder(&self.device)
    }

    pub fn create_material(&self) -> MaterialBuilder {
        let default_shader = self
            .builtin_shaders
            .get(&BuiltinShader::Texture)
            .or_error("builtins not setup");
        let default_texture = self
            .builtin_textures
            .get(&BuiltinTexture::White)
            .or_error("builtins not setup");
        Material::builder(
            &self.device,
            default_shader,
            default_texture,
            &self.shader_layout,
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
        let device = Device::new(&vulkan, &window_surface, &extensions, self.vsync, 0);
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
        let mut render_passes = HashMap::new();
        render_passes.insert(
            RenderPassType::ColorOnscreen,
            RenderPass::color_onscreen(&device),
        );
        render_passes.insert(
            RenderPassType::ColorOffscreen,
            RenderPass::color_offscreen(&device),
        );
        render_passes.insert(
            RenderPassType::DepthOffscreen,
            RenderPass::depth_offscreen(&device),
        );
        info!("render passes created");

        debug!("create builtins");
        let builtin_shaders = HashMap::new();
        let builtin_textures = HashMap::new();
        let builtin_meshes = HashMap::new();
        let builtin_materials = HashMap::new();
        info!("builtins created");

        Tegne {
            builtin_shaders,
            builtin_textures,
            builtin_meshes,
            builtin_materials,
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
