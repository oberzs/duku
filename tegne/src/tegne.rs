// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Tegne - tegne application entrypoint

use crossbeam_channel::bounded;
use crossbeam_channel::select;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::camera::Camera;
use crate::camera::CameraType;
use crate::color::Color;
use crate::device::pick_gpu;
use crate::device::Device;
use crate::device::DeviceProperties;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::Texture;
use crate::image::TextureFormat;
use crate::image::TextureOptions;
use crate::instance::Instance;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::AttachmentType;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::MaterialOptions;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::renderer::ForwardRenderer;
use crate::renderer::RenderStats;
use crate::renderer::Target;
use crate::resource::Builtins;
use crate::resource::Ref;
use crate::resource::ResourceManager;
use crate::surface::Surface;
use crate::surface::SurfaceProperties;
use crate::surface::Swapchain;
use crate::surface::WindowHandle;

#[cfg(feature = "ui")]
use crate::renderer::UiRenderer;

#[cfg(feature = "window")]
use crate::surface::Window;

// TODO: remove and pass errors to user
macro_rules! check {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => error!("{}", err),
        }
    };
}

pub struct Tegne {
    pub main_camera: Camera,
    render_stage: RenderStage,
    thread_kill: ThreadKill,
    forward_renderer: ForwardRenderer,
    #[cfg(feature = "ui")]
    ui_renderer: UiRenderer,
    #[cfg(feature = "ui")]
    is_ui_rendered: bool,
    builtins: Builtins,
    resources: ResourceManager,
    window_framebuffers: Arc<Mutex<Vec<Framebuffer>>>,
    image_uniform: ImageUniform,
    shader_layout: Arc<ShaderLayout>,
    swapchain: Swapchain,
    device: Arc<Device>,
    surface: Surface,
    gpu_index: usize,
    instance: Arc<Instance>,
    camera_type: CameraType,
    render_stats: RenderStats,
}

#[derive(Debug, Copy, Clone)]
pub struct TegneOptions {
    pub anisotropy: f32,
    pub vsync: bool,
    pub msaa: u8,
    pub camera: CameraType,
}

#[derive(Copy, Clone)]
enum RenderStage {
    Before,
    During,
}

struct ThreadKill {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl Tegne {
    pub fn new(window: WindowHandle, options: TegneOptions) -> Self {
        profile_scope!("new");

        let instance = Arc::new(check!(Instance::new()));
        let surface = check!(Surface::new(&instance, window));

        // query GPU properties
        let mut surface_properties_list =
            check!(SurfaceProperties::new(&instance, &surface, options.vsync));
        let mut device_properties_list = check!(DeviceProperties::new(&instance, options.msaa));

        // pick GPU
        let gpu_index = check!(pick_gpu(&surface_properties_list, &device_properties_list));
        let surface_properties = surface_properties_list.remove(gpu_index);
        let device_properties = device_properties_list.remove(gpu_index);

        let device = Arc::new(check!(Device::new(
            &instance,
            &surface_properties,
            device_properties,
            gpu_index
        )));

        let swapchain = check!(Swapchain::new(&device, &surface, surface_properties));

        let shader_layout = check!(ShaderLayout::new(&device));

        let image_uniform = check!(ImageUniform::new(
            &device,
            &shader_layout,
            options.anisotropy
        ));

        let window_framebuffers = check!(Framebuffer::for_swapchain(
            &device,
            &swapchain,
            &shader_layout,
            FramebufferOptions {
                attachment_types: &[AttachmentType::Depth, AttachmentType::Color],
                camera_type: options.camera,

                // does not matter for window framebuffers
                multisampled: false,
                width: 1,
                height: 1,
            }
        ));

        let mut resources = ResourceManager::new();
        let builtins = check!(Builtins::new(
            &device,
            &mut resources,
            &window_framebuffers[0],
            &shader_layout,
            &image_uniform
        ));

        let forward_renderer = check!(ForwardRenderer::new(&device, &shader_layout));
        #[cfg(feature = "ui")]
        let ui_renderer = check!(UiRenderer::new(
            &device,
            &shader_layout,
            &mut resources,
            window.width,
            window.height
        ));

        let main_camera = Camera::new(
            options.camera,
            window.width as f32,
            window.height as f32,
            100.0,
        );

        Self {
            render_stage: RenderStage::Before,
            thread_kill: ThreadKill::new(),
            forward_renderer,
            builtins,
            resources,
            window_framebuffers: Arc::new(Mutex::new(window_framebuffers)),
            image_uniform,
            shader_layout: Arc::new(shader_layout),
            swapchain,
            device,
            surface,
            gpu_index,
            instance,
            main_camera,
            camera_type: options.camera,
            render_stats: Default::default(),

            #[cfg(feature = "ui")]
            ui_renderer,
            #[cfg(feature = "ui")]
            is_ui_rendered: false,
        }
    }

    #[cfg(feature = "window")]
    pub fn from_window(window: &mut Window, options: TegneOptions) -> Self {
        let (width, height) = window.size();

        #[cfg(target_os = "windows")]
        let handle = WindowHandle {
            hwnd: window.hwnd(),
            width,
            height,
        };

        #[cfg(target_os = "linux")]
        let handle = WindowHandle {
            xlib_window: window.xlib_window(),
            xlib_display: window.xlib_display(),
            width,
            height,
        };

        #[cfg(target_os = "macos")]
        let handle = WindowHandle {
            ns_window: window.ns_window(),
            ns_view: window.ns_view(),
            width,
            height,
        };

        let mut s = Self::new(handle, options);

        #[cfg(feature = "ui")]
        {
            let ui_texture = window.build_ui_texture();
            check!(s.ui_renderer.set_font_texture(&s.image_uniform, ui_texture));
        }

        s
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        check!(self.device.wait_for_idle());
        self.surface.resize(width, height);
        self.main_camera.width = width as f32;
        self.main_camera.height = height as f32;
        check!(self
            .swapchain
            .recreate(&self.instance, &self.surface, self.gpu_index));

        let mut framebuffers = self.window_framebuffers.lock().unwrap();
        *framebuffers = check!(Framebuffer::for_swapchain(
            &self.device,
            &self.swapchain,
            &self.shader_layout,
            FramebufferOptions {
                attachment_types: &[AttachmentType::Depth, AttachmentType::Color],
                camera_type: self.camera_type,

                // does not matter for window framebuffers
                multisampled: false,
                width: 1,
                height: 1,
            }
        ));

        #[cfg(feature = "ui")]
        self.ui_renderer.resize(&self.shader_layout, width, height);
    }

    pub fn draw_on_window(&mut self, draw_callback: impl Fn(&mut Target)) {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        {
            let mut target = check!(Target::new(&self.builtins));
            draw_callback(&mut target);

            #[cfg(feature = "ui")]
            if self.is_ui_rendered {
                target.blit_framebuffer(self.ui_renderer.framebuffer());
            }

            let framebuffer =
                &mut self.window_framebuffers.lock().unwrap()[self.swapchain.current()];
            framebuffer.camera = self.main_camera.clone();

            self.render_stats += check!(self.forward_renderer.draw(
                &self.device,
                framebuffer,
                &self.shader_layout,
                target,
            ));
        }

        self.end_draw();
    }

    pub fn draw(&mut self, framebuffer: &Ref<Framebuffer>, draw_callback: impl Fn(&mut Target)) {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        let mut target = check!(Target::new(&self.builtins));
        draw_callback(&mut target);

        self.render_stats += framebuffer.with(|f| {
            check!(self
                .forward_renderer
                .draw(&self.device, f, &self.shader_layout, target))
        });
    }

    #[cfg(feature = "ui")]
    pub fn draw_ui(&mut self, ui: imgui::Ui<'_>) {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        check!(self.ui_renderer.draw(ui, &self.shader_layout));
        self.is_ui_rendered = true;
    }

    pub fn create_texture(&mut self, pixels: &[Color], width: u32) -> Ref<Texture> {
        let data = pixels
            .iter()
            .map(|p| vec![p.r, p.g, p.b, p.a])
            .flatten()
            .collect::<Vec<_>>();
        let texture = check!(Texture::new(
            &self.device,
            &self.image_uniform,
            TextureOptions {
                format: TextureFormat::Rgba,
                data: &data,
                height: pixels.len() as u32 / width,
                width,
            }
        ));
        self.resources.add_texture(texture)
    }

    #[cfg(feature = "image")]
    pub fn create_texture_from_file(&mut self, path: impl AsRef<Path>) -> Result<Ref<Texture>> {
        use image_file::GenericImageView;
        let img = image_file::open(path)?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();

        let texture = check!(Texture::new(
            &self.device,
            &self.image_uniform,
            TextureOptions {
                format: TextureFormat::Srgba,
                data: &data,
                width,
                height,
            }
        ));
        Ok(self.resources.add_texture(texture))
    }

    pub fn create_mesh(&mut self, options: MeshOptions<'_>) -> Ref<Mesh> {
        let mesh = check!(Mesh::new(&self.device, options));
        self.resources.add_mesh(mesh)
    }

    pub fn combine_meshes(&mut self, meshes: &[Ref<Mesh>]) -> Ref<Mesh> {
        let mut offset = 0;
        let mut triangles = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut colors = vec![];
        for mesh in meshes {
            mesh.with(|m| {
                triangles.extend(
                    m.triangles()
                        .iter()
                        .map(|t| [t[0] + offset, t[1] + offset, t[2] + offset]),
                );
                vertices.extend(m.vertices());
                normals.extend(m.normals());
                uvs.extend(m.uvs());
                colors.extend(m.colors());
                offset = vertices.len() as u32;
            });
        }

        let mesh = check!(Mesh::new(
            &self.device,
            MeshOptions {
                vertices: &vertices,
                normals: &normals,
                uvs: &uvs,
                colors: &colors,
                triangles: &triangles,
            }
        ));
        self.resources.add_mesh(mesh)
    }

    pub fn create_material(&mut self, options: MaterialOptions) -> Ref<Material> {
        let material = check!(Material::new(&self.device, &self.shader_layout, options));
        self.resources.add_material(material)
    }

    pub fn create_framebuffer(
        &mut self,
        t: CameraType,
        width: u32,
        height: u32,
    ) -> Ref<Framebuffer> {
        let framebuffer = check!(Framebuffer::new(
            &self.device,
            &self.shader_layout,
            FramebufferOptions {
                attachment_types: &[AttachmentType::Depth, AttachmentType::Color],
                camera_type: t,
                multisampled: self.device.is_msaa(),
                width,
                height,
            }
        ));
        self.resources.add_framebuffer(framebuffer)
    }

    pub fn resize_framebuffer(&self, framebuffer: &Ref<Framebuffer>, width: u32, height: u32) {
        framebuffer.with(|f| {
            check!(f.resize(width, height, &self.shader_layout));
        });
    }

    pub fn create_shader(&mut self, source: &[u8], options: ShaderOptions) -> Ref<Shader> {
        let framebuffer = &self.window_framebuffers.lock().unwrap()[0];
        let shader = check!(Shader::new(
            &self.device,
            framebuffer,
            &self.shader_layout,
            source,
            options,
        ));
        self.resources.add_shader(shader)
    }

    pub fn create_shader_from_file(
        &mut self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Result<Ref<Shader>> {
        let source = fs::read(path.as_ref())?;
        Ok(self.create_shader(&source, options))
    }

    pub fn create_shader_from_file_watch(
        &mut self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Result<Ref<Shader>> {
        let path_buf = path.as_ref().to_path_buf();
        let shader = self.create_shader_from_file(&path_buf, options)?;

        // setup watcher
        let framebuffers = self.window_framebuffers.clone();
        let shader_layout = self.shader_layout.clone();
        let device = self.device.clone();
        let shader_ref = shader.clone();
        let kill_recv = self.thread_kill.receiver();

        thread::spawn(move || {
            let (sender, receiver) = unbounded();
            let start_time = Instant::now();
            let mut watcher: RecommendedWatcher = check!(Watcher::new_immediate(move |res| {
                let time = start_time.elapsed().as_secs();
                sender.send((check!(res), time)).unwrap()
            }));
            check!(watcher.watch(&path_buf, RecursiveMode::NonRecursive));

            let mut same_events = HashSet::new();
            loop {
                select! {
                    recv(kill_recv) -> _ => break,
                    recv(receiver) -> signal => if let Ok((_, time)) = signal {
                        // limit events
                        if !same_events.contains(&time) {
                            same_events.insert(time);

                            // wait to commit
                            thread::sleep(Duration::from_millis(500));

                            let framebuffer = &framebuffers.lock().unwrap()[0];

                            let source = check!(fs::read(&path_buf));
                            let new_shader = check!(Shader::new(
                                &device,
                                framebuffer,
                                &shader_layout,
                                &source,
                                options,
                            ));
                            shader_ref.with(|s| *s = new_shader);
                            info!("shader {:?} was reloaded", path_buf);
                        }
                    }
                }
            }
        });

        Ok(shader)
    }

    pub fn render_stats(&self) -> RenderStats {
        self.render_stats
    }

    fn begin_draw(&mut self) {
        self.render_stage = RenderStage::During;
        self.render_stats = Default::default();
        check!(self.device.next_frame(&self.swapchain));
        self.resources.clean_unused(&self.image_uniform);
        self.image_uniform.update_if_needed();
        self.device.cmd_bind_descriptor(
            self.device.command_buffer(),
            self.image_uniform.descriptor(),
            &self.shader_layout,
        );

        #[cfg(feature = "ui")]
        {
            self.is_ui_rendered = false;
        }
    }

    fn end_draw(&mut self) {
        self.render_stage = RenderStage::Before;
        check!(self.device.submit());
        check!(self.device.present(&self.swapchain));
    }
}

impl Drop for Tegne {
    fn drop(&mut self) {
        self.thread_kill.kill().unwrap();
        self.device.wait_for_idle().unwrap();
    }
}

impl Default for TegneOptions {
    fn default() -> Self {
        Self {
            anisotropy: 0.0,
            vsync: true,
            msaa: 1,
            camera: CameraType::Perspective,
        }
    }
}

impl ThreadKill {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = bounded(1);
        Self { sender, receiver }
    }

    pub(crate) fn receiver(&self) -> Receiver<()> {
        self.receiver.clone()
    }

    pub(crate) fn kill(&self) -> Result<()> {
        self.sender.send(())?;
        Ok(())
    }
}
