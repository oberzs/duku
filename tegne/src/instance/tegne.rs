use image::GenericImageView;
use log::debug;
use notify::DebouncedEvent;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::cell::RefMut;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tegne_math::Camera;

use super::Device;
use super::Extensions;
use super::Surface;
use super::Swapchain;
use super::Target;
use super::Validator;
use super::Vulkan;
use super::WindowArgs;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::objects::Id;
use crate::objects::Objects;
use crate::renderer::ForwardDrawOptions;
use crate::renderer::ForwardRenderer;
use crate::shaders::ImageUniforms;
use crate::shaders::Material;
use crate::shaders::MaterialOptions;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::shaders::ShaderOptions;
use crate::utils::OrError;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    start_time: Instant,
    forward_renderer: ForwardRenderer,
    objects: Objects,
    window_framebuffers: Vec<Framebuffer>,
    render_passes: HashMap<RenderPassType, RenderPass>,
    image_uniforms: ImageUniforms,
    shader_layout: ShaderLayout,
    swapchain: Swapchain,
    device: Arc<Device>,
    _surface: Surface,
    _validator: Option<Validator>,
    _vulkan: Vulkan,
}

#[derive(Debug, Copy, Clone)]
pub struct TegneOptions {
    pub anisotropy: f32,
    pub vsync: bool,
    pub msaa: u8,
}

#[derive(Hash, Eq, PartialEq)]
pub(crate) enum RenderPassType {
    Window,
    Color,
    Depth,
}

impl Tegne {
    pub fn new(window: WindowArgs, options: TegneOptions) -> Self {
        let extensions = Extensions::new();

        let vulkan = Vulkan::new(&extensions).or_error("cannot initialize Vulkan");

        #[cfg(debug_assertions)]
        let validator = Some(Validator::new(&vulkan).or_error("cannot create validator"));
        #[cfg(not(debug_assertions))]
        let validator = None;

        let surface = Surface::new(&vulkan, window).or_error("cannot create surface");

        let device = Device::new(&vulkan, &surface, &extensions, options.vsync, options.msaa)
            .or_error("cannot initialize device");

        let swapchain =
            Swapchain::new(&vulkan, &device, &surface).or_error("cannot create swapchain");

        let shader_layout = ShaderLayout::new(&device).or_error("cannot create shader layout");

        let image_uniforms = ImageUniforms::new(&device, &shader_layout, options.anisotropy)
            .or_error("cannot create image uniforms");

        let mut render_passes = HashMap::new();
        render_passes.insert(
            RenderPassType::Color,
            RenderPass::color(&device).or_error("cannot create render pass"),
        );
        render_passes.insert(
            RenderPassType::Window,
            RenderPass::window(&device).or_error("cannot create render pass"),
        );
        render_passes.insert(
            RenderPassType::Depth,
            RenderPass::depth(&device).or_error("cannot create render pass"),
        );

        let objects = Objects::new(&device, &render_passes, &shader_layout, &image_uniforms)
            .or_error("cannot create object storage");

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
        )
        .or_error("cannot create window framebuffers");

        let forward_renderer =
            ForwardRenderer::new(&device, &depth_pass, &image_uniforms, &shader_layout)
                .or_error("cannot create forward renderer");

        Self {
            start_time: Instant::now(),
            forward_renderer,
            objects,
            window_framebuffers,
            render_passes,
            image_uniforms,
            shader_layout,
            swapchain,
            device,
            _surface: surface,
            _validator: validator,
            _vulkan: vulkan,
        }
    }

    #[cfg(feature = "tegne-utils")]
    pub fn from_window(window: &Window, options: TegneOptions) -> Self {
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

        Self::new(args, options)
    }

    pub fn begin_draw(&self) {
        self.device
            .next_frame(&self.swapchain)
            .or_error("cannot start next frame");
        self.image_uniforms
            .update_if_needed()
            .or_error("cannot update image uniforms");
        self.device
            .commands()
            .bind_descriptor(
                self.image_uniforms.descriptor(),
                self.shader_layout.pipeline(),
            )
            .or_error("cannod bind descriptor");
    }

    pub fn end_draw(&self) {
        self.device.submit().or_error("cannot submit");
        self.device
            .present(&self.swapchain)
            .or_error("cannot present");
    }

    pub fn draw_on_window(&self, camera: &Camera, draw_callback: impl Fn(&mut Target<'_>)) {
        let mut target = Target::new(&self.objects).or_error("cannot create target");
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

        self.forward_renderer
            .draw(ForwardDrawOptions {
                framebuffer,
                color_pass: window_pass,
                depth_pass,
                shader_layout: &self.shader_layout,
                camera,
                objects: &self.objects,
                target,
                time: self.start_time.elapsed().as_secs_f32(),
            })
            .or_error("cannot do forward render");
    }

    pub fn create_texture_rgba(&self, raw: &[u8], width: u32, height: u32) -> Id<Texture> {
        debug!("creating rgba texture");
        let texture =
            Texture::from_raw_rgba(&self.device, raw, width, height, &self.image_uniforms)
                .or_error("cannot create texture");
        self.objects.add_texture(texture)
    }

    pub fn create_texture_rgb(&self, raw: &[u8], width: u32, height: u32) -> Id<Texture> {
        debug!("creating rgb texture");
        let texture = Texture::from_raw_rgb(&self.device, raw, width, height, &self.image_uniforms)
            .or_error("cannot create texture");
        self.objects.add_texture(texture)
    }

    pub fn create_texture_from_file(&self, path: impl AsRef<Path>) -> Id<Texture> {
        let p = path.as_ref();
        let img = image::open(p).or_error(format!("cannot open image {}", p.display()));
        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();
        self.create_texture_rgba(&data, width, height)
    }

    pub fn create_mesh(&self, options: MeshOptions<'_>) -> Id<Mesh> {
        debug!("creating mesh");
        let mesh = Mesh::new(&self.device, options).or_error("cannot create mesh");
        self.objects.add_mesh(mesh)
    }

    pub fn create_material(&self, options: MaterialOptions) -> Id<Material> {
        debug!("creating material");
        let material = Material::new(&self.device, &self.shader_layout, options)
            .or_error("cannot create material");
        self.objects.add_material(material)
    }

    pub fn get_material(&self, material: Id<Material>) -> RefMut<'_, Material> {
        self.objects.material(material)
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
        .or_error("cannot create framebuffer")
    }

    pub fn create_shader(&self, source: &[u8], options: ShaderOptions) -> Id<Shader> {
        debug!("creating shader");
        let render_pass = self
            .render_passes
            .get(&RenderPassType::Color)
            .or_error("render passes not setup");
        let shader = Shader::new(
            &self.device,
            render_pass,
            &self.shader_layout,
            source,
            options,
        )
        .or_error("cannot create shader");
        self.objects.add_shader(shader)
    }

    pub fn create_shader_from_file_watch(
        &self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Id<Shader> {
        let path_buf = path.as_ref().to_path_buf();
        let source =
            fs::read(&path_buf).or_error(format!("cannot open shader {}", path_buf.display()));
        let id = self.create_shader(&source, options);

        // setup watcher
        let arc_path_buf = path_buf;
        thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher: RecommendedWatcher =
                Watcher::new(tx, Duration::from_secs(1)).or_error("cannot watch system");
            watcher
                .watch(&arc_path_buf, RecursiveMode::NonRecursive)
                .expect("cannot watch shader");
            loop {
                let event = rx.recv().unwrap();
                if let DebouncedEvent::NoticeWrite(_) = event {
                    println!("new shader {:?}", &arc_path_buf);
                    // let new_source = fs::read(&new_path)
                    // .or_error(format!("cannot open shader {}", new_path.display()));
                    // let render_pass = self
                    //     .render_passes
                    //     .get(&RenderPassType::Color)
                    //     .or_error("render passes not setup");
                    // let shader = Shader::new(
                    //     &self.device,
                    //     render_pass,
                    //     &self.shader_layout,
                    //     &new_source,
                    //     options,
                    // );
                }
            }
        });

        id
    }
}

impl Drop for Tegne {
    fn drop(&mut self) {
        self.device.wait_for_idle().unwrap();
    }
}

impl Default for TegneOptions {
    fn default() -> Self {
        Self {
            anisotropy: 0.0,
            vsync: false,
            msaa: 1,
        }
    }
}
