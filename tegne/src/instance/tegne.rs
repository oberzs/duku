use image::GenericImageView;
use log::debug;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;
use tegne_math::Camera;

use super::Device;
use super::Extensions;
use super::Swapchain;
use super::Target;
use super::Validator;
use super::Vulkan;
use super::WindowArgs;
use super::WindowSurface;
use crate::builtins::Builtins;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::mesh::MeshBuilder;
use crate::renderer::ForwardDrawOptions;
use crate::renderer::ForwardRenderer;
use crate::shaders::ImageUniforms;
use crate::shaders::Material;
use crate::shaders::MaterialBuilder;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderBuilder;
use crate::shaders::ShaderLayout;
use crate::utils::OrError;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    start_time: Instant,
    forward_renderer: ForwardRenderer,
    builtins: Builtins,
    window_framebuffers: Vec<Framebuffer>,
    render_passes: HashMap<RenderPassType, RenderPass>,
    image_uniforms: ImageUniforms,
    shader_layout: ShaderLayout,
    swapchain: Swapchain,
    device: Rc<Device>,
    _window_surface: WindowSurface,
    _validator: Option<Validator>,
    _vulkan: Vulkan,
}

pub struct TegneBuilder {
    window_args: Option<WindowArgs>,
    anisotropy: f32,
    vsync: bool,
    msaa: u8,
}

#[derive(Hash, Eq, PartialEq)]
pub(crate) enum RenderPassType {
    Window,
    Color,
    Depth,
}

impl Tegne {
    pub fn builder() -> TegneBuilder {
        TegneBuilder {
            window_args: None,
            anisotropy: 0.0,
            vsync: false,
            msaa: 1,
        }
    }

    pub fn begin_draw(&self) {
        self.device.next_frame(&self.swapchain);
        self.image_uniforms.update_if_needed();
        self.device.record_commands().bind_descriptor(
            self.image_uniforms.descriptor(),
            self.shader_layout.pipeline(),
        );
    }

    pub fn end_draw(&self) {
        self.device.submit();
        self.device.present(&self.swapchain);
    }

    pub fn draw_on_window(&self, camera: &Camera, draw_callback: impl Fn(&mut Target<'_>)) {
        let mut target = Target::new(&self.builtins);
        draw_callback(&mut target);

        let framebuffer = &self.window_framebuffers[self.swapchain.current()];
        let window_pass = self
            .render_passes
            .get(&RenderPassType::Window)
            .or_error("render passes not setup");
        let depth_pass = self
            .render_passes
            .get(&RenderPassType::Depth)
            .or_error("render passes not setup");

        self.forward_renderer.draw(ForwardDrawOptions {
            framebuffer,
            color_pass: window_pass,
            depth_pass,
            shader_layout: &self.shader_layout,
            camera,
            builtins: &self.builtins,
            target,
            time: self.start_time.elapsed().as_secs_f32(),
        });
    }

    pub fn create_texture_rgba(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("creating rgba texture");
        Texture::from_raw_rgba(&self.device, raw, width, height, &self.image_uniforms)
    }

    pub fn create_texture_rgb(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("creating rgb texture");
        Texture::from_raw_rgb(&self.device, raw, width, height, &self.image_uniforms)
    }

    pub fn create_texture_from_file(&self, path: impl AsRef<Path>) -> Texture {
        let p = path.as_ref();
        let img = image::open(p).or_error(format!("cannot open image {}", p.display()));
        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();
        self.create_texture_rgba(&data, width, height)
    }

    pub fn create_mesh(&self) -> MeshBuilder {
        debug!("creating mesh");
        Mesh::builder(&self.device)
    }

    pub fn create_material(&self) -> MaterialBuilder {
        debug!("creating material");
        Material::builder(&self.device, &self.shader_layout)
    }

    pub fn create_framebuffer(&self, width: u32, height: u32) -> Framebuffer {
        debug!("creating framebuffer");
        let render_pass = self
            .render_passes
            .get(&RenderPassType::Color)
            .or_error("render passes not setup");
        Framebuffer::color(
            &self.device,
            render_pass,
            &self.image_uniforms,
            &self.shader_layout,
            width,
            height,
        )
    }

    pub fn create_shader(&self) -> ShaderBuilder {
        debug!("creating shader");
        let render_pass = self
            .render_passes
            .get(&RenderPassType::Color)
            .or_error("render passes not setup");
        Shader::builder(&self.device, render_pass, &self.shader_layout)
    }
}

impl Drop for Tegne {
    fn drop(&mut self) {
        self.device.wait_for_idle();
    }
}

impl TegneBuilder {
    pub fn build(&self) -> Tegne {
        let window_args = self.window_args.or_error("window arguments not set");
        let extensions = Extensions::new();

        let vulkan = Vulkan::new(&extensions);

        #[cfg(debug_assertions)]
        let validator = Some(Validator::new(&vulkan));
        #[cfg(not(debug_assertions))]
        let validator = None;

        let window_surface = WindowSurface::new(&vulkan, window_args);

        let device = Device::new(&vulkan, &window_surface, &extensions, self.vsync, self.msaa);

        let swapchain = Swapchain::new(
            &vulkan,
            &device,
            &window_surface,
            window_args.width,
            window_args.height,
        );

        let shader_layout = ShaderLayout::new(&device);

        let image_uniforms = ImageUniforms::new(&device, &shader_layout, self.anisotropy);

        let mut render_passes = HashMap::new();
        render_passes.insert(RenderPassType::Color, RenderPass::color(&device));
        render_passes.insert(RenderPassType::Window, RenderPass::window(&device));
        render_passes.insert(RenderPassType::Depth, RenderPass::depth(&device));

        let builtins = Builtins::new(&device, &render_passes, &shader_layout, &image_uniforms);

        let window_pass = render_passes
            .get(&RenderPassType::Window)
            .or_error("render passes not setup");
        let depth_pass = render_passes
            .get(&RenderPassType::Depth)
            .or_error("render passes not setup");

        let window_framebuffers = Framebuffer::window(
            &device,
            &swapchain,
            &window_pass,
            &image_uniforms,
            &shader_layout,
            window_args.width,
            window_args.height,
        );

        let forward_renderer =
            ForwardRenderer::new(&device, &depth_pass, &image_uniforms, &shader_layout);

        Tegne {
            start_time: Instant::now(),
            forward_renderer,
            builtins,
            window_framebuffers,
            render_passes,
            image_uniforms,
            shader_layout,
            swapchain,
            device,
            _window_surface: window_surface,
            _validator: validator,
            _vulkan: vulkan,
        }
    }

    #[cfg(feature = "tegne-utils")]
    pub fn with_window(&mut self, window: &Window) -> &mut Self {
        let (width, height) = window.size();

        #[cfg(target_os = "windows")]
        let args = WindowArgs {
            hwnd: window.hwnd(),
            width,
            height,
        };

        #[cfg(target_os = "linux")]
        let args = WindowArgs {
            xlib_window: window.xlib_window(),
            xlib_display: window.xlib_display(),
            width,
            height,
        };

        #[cfg(target_os = "macos")]
        let args = WindowArgs {
            ns_window: window.ns_window(),
            ns_view: window.ns_view(),
            width,
            height,
        };

        self.window_args = Some(args);
        self
    }

    pub fn with_window_args(&mut self, value: WindowArgs) -> &mut Self {
        self.window_args = Some(value);
        self
    }

    pub fn with_anisotropy(&mut self, value: f32) -> &mut Self {
        self.anisotropy = value;
        self
    }

    pub fn with_vsync(&mut self) -> &mut Self {
        self.vsync = true;
        self
    }

    pub fn with_msaa(&mut self, value: u8) -> &mut Self {
        self.msaa = value;
        self
    }
}
