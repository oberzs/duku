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
use crate::model::create_cube;
use crate::model::create_sphere;
use crate::model::Material;
use crate::model::MaterialBuilder;
use crate::model::Mesh;
use crate::model::MeshBuilder;
use crate::shaders::CullMode;
use crate::shaders::Depth;
use crate::shaders::FragmentMode;
use crate::shaders::ImageUniforms;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

macro_rules! include_shader {
    ($path:expr) => {
        include_bytes!(concat!(env!("OUT_DIR"), "/shaders/", $path, ".spv")).as_ref()
    };
}

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

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum BuiltinShader {
    Phong,
    Unshaded,
    Passthru,
    Wireframe,
    Shadow,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
enum BuiltinTexture {
    White,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum BuiltinMesh {
    Cube,
    Sphere,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
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
            .get(&BuiltinShader::Phong)
            .or_error("shader builtins not setup");
        let default_texture = self
            .builtin_textures
            .get(&BuiltinTexture::White)
            .or_error("texture builtins not setup");
        Material::builder(
            &self.device,
            default_shader,
            default_texture,
            &self.shader_layout,
        )
    }

    pub fn get_mesh(&self, mesh: BuiltinMesh) -> &Mesh {
        self.builtin_meshes
            .get(&mesh)
            .or_error("mesh builtins not setup")
    }

    pub fn get_cube_mesh(&self) -> &Mesh {
        self.get_mesh(BuiltinMesh::Cube)
    }

    pub fn get_sphere_mesh(&self) -> &Mesh {
        self.get_mesh(BuiltinMesh::Sphere)
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

        debug!("create builtin shaders");
        let world_vert = include_shader!("world.vert");
        let passthru_vert = include_shader!("passthru.vert");
        let shadow_vert = include_shader!("shadow.vert");
        let phong_frag = include_shader!("phong.frag");
        let wireframe_frag = include_shader!("wireframe.frag");
        let passthru_frag = include_shader!("passthru.frag");
        let shadow_frag = include_shader!("shadow.frag");

        let phong_shader = Shader::new(
            &device,
            &c_render_pass,
            world_vert,
            phong_frag,
            FragmentMode::Fill,
            CullMode::Back,
            Depth::Enabled,
            &shader_layout,
        );
        let unshaded_shader = Shader::new(
            &device,
            &c_render_pass,
            world_vert,
            passthru_frag,
            FragmentMode::Fill,
            CullMode::Back,
            Depth::Enabled,
            &shader_layout,
        );
        let passthru_shader = Shader::new(
            &device,
            &c_render_pass,
            passthru_vert,
            passthru_frag,
            FragmentMode::Fill,
            CullMode::Back,
            Depth::Disabled,
            &shader_layout,
        );
        let shadow_shader = Shader::new(
            &device,
            &d_render_pass,
            shadow_vert,
            shadow_frag,
            FragmentMode::Fill,
            CullMode::Back,
            Depth::Enabled,
            &shader_layout,
        );
        let wireframe_shader = Shader::new(
            &device,
            &c_render_pass,
            world_vert,
            wireframe_frag,
            FragmentMode::Lines,
            CullMode::Back,
            Depth::Disabled,
            &shader_layout,
        );
        info!("builtin shaders created");

        debug!("create builtin textures");
        let builtin_textures = HashMap::new();
        info!("builtin textures created");

        debug!("create builtin meshes");
        let cube_mesh = create_cube(&device);
        let sphere_mesh = create_sphere(&device, 2);
        info!("builtin meshes created");

        debug!("create builtin materials");
        let builtin_materials = HashMap::new();
        info!("builtin materials created");

        let mut render_passes = HashMap::new();
        render_passes.insert(RenderPassType::ColorOffscreen, c_render_pass);
        render_passes.insert(RenderPassType::DepthOffscreen, d_render_pass);

        let mut builtin_shaders = HashMap::new();
        builtin_shaders.insert(BuiltinShader::Phong, phong_shader);
        builtin_shaders.insert(BuiltinShader::Unshaded, unshaded_shader);
        builtin_shaders.insert(BuiltinShader::Passthru, passthru_shader);
        builtin_shaders.insert(BuiltinShader::Wireframe, wireframe_shader);
        builtin_shaders.insert(BuiltinShader::Shadow, shadow_shader);

        let mut builtin_meshes = HashMap::new();
        builtin_meshes.insert(BuiltinMesh::Cube, cube_mesh);
        builtin_meshes.insert(BuiltinMesh::Sphere, sphere_mesh);

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
