use crossbeam::channel;
use crossbeam::channel::select;
use image::GenericImageView;
use log::debug;
use log::error;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tegne_math::Camera;

use super::Commands;
use super::Device;
use super::Extensions;
use super::Surface;
use super::Swapchain;
use super::Target;
use super::ThreadKill;
use super::Validator;
use super::Vulkan;
use super::WindowArgs;
use super::IN_FLIGHT_FRAME_COUNT;
use crate::error::Result;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::objects::Builtins;
use crate::objects::Id;
use crate::objects::Objects;
use crate::renderer::ForwardDrawOptions;
use crate::renderer::ForwardRenderer;
use crate::shaders::ImageUniforms;
use crate::shaders::Material;
use crate::shaders::MaterialOptions;
use crate::shaders::RenderPasses;
use crate::shaders::Shader;
use crate::shaders::ShaderLayout;
use crate::shaders::ShaderOptions;

macro_rules! check {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => panic!(error!("{}", err)),
        }
    };
}

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    render_stage: RenderStage,
    thread_kill: ThreadKill,
    start_time: Instant,
    forward_renderer: ForwardRenderer,
    builtins: Builtins,
    objects: Arc<Objects>,
    window_framebuffers: Vec<Framebuffer>,
    commands: Vec<Commands>,
    render_passes: Arc<RenderPasses>,
    image_uniforms: ImageUniforms,
    shader_layout: Arc<ShaderLayout>,
    swapchain: Swapchain,
    device: Arc<Device>,
    surface: Surface,
    _validator: Option<Validator>,
    _vulkan: Vulkan,
}

#[derive(Debug, Copy, Clone)]
pub struct TegneOptions {
    pub anisotropy: f32,
    pub vsync: bool,
    pub msaa: u8,
}

#[derive(Copy, Clone)]
enum RenderStage {
    Before,
    During,
}

impl Tegne {
    pub fn new(window: WindowArgs, options: TegneOptions) -> Self {
        let extensions = check!(Extensions::new());

        let vulkan = check!(Vulkan::new(&extensions));

        #[cfg(debug_assertions)]
        let validator = Some(check!(Validator::new(&vulkan)));
        #[cfg(not(debug_assertions))]
        let validator = None;

        let surface = check!(Surface::new(&vulkan, window));

        let device = check!(Device::new(
            &vulkan,
            &surface,
            &extensions,
            options.vsync,
            options.msaa
        ));

        let swapchain = check!(Swapchain::new(&vulkan, &device, &surface));

        let shader_layout = check!(ShaderLayout::new(&device));

        let image_uniforms = check!(ImageUniforms::new(
            &device,
            &shader_layout,
            options.anisotropy
        ));

        let render_passes = check!(RenderPasses::new(&device));

        let objects = Objects::new();

        let builtins = check!(Builtins::new(
            &device,
            &render_passes,
            &shader_layout,
            &image_uniforms,
            &objects,
        ));

        let window_framebuffers = check!(Framebuffer::window(
            &device,
            &swapchain,
            &render_passes,
            &shader_layout,
        ));

        let commands = (0..IN_FLIGHT_FRAME_COUNT)
            .map(|_| check!(Commands::new(&device)))
            .collect::<Vec<_>>();

        let forward_renderer = check!(ForwardRenderer::new(
            &device,
            &render_passes,
            &image_uniforms,
            &shader_layout
        ));

        Self {
            render_stage: RenderStage::Before,
            thread_kill: ThreadKill::new(),
            start_time: Instant::now(),
            forward_renderer,
            builtins,
            objects: Arc::new(objects),
            window_framebuffers,
            commands,
            render_passes: Arc::new(render_passes),
            image_uniforms,
            shader_layout: Arc::new(shader_layout),
            swapchain,
            device,
            surface,
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

    pub fn resize(&mut self, width: u32, height: u32) {
        check!(self.device.wait_for_idle());
        self.surface.resize(width, height);
        check!(self.device.refresh_for_surface(&self.surface));
        check!(self.swapchain.recreate(&self.device, &self.surface));
        self.window_framebuffers = check!(Framebuffer::window(
            &self.device,
            &self.swapchain,
            &self.render_passes,
            &self.shader_layout,
        ));
    }

    pub fn begin_draw(&mut self) {
        if let RenderStage::During = self.render_stage {
            panic!(error!("cannot begin draw stage during draw stage"));
        } else {
            self.render_stage = RenderStage::During;
        }

        check!(self.device.next_frame(&self.swapchain));
        self.image_uniforms.update_if_needed();
        let current = self.device.current_frame();
        let cmd = &mut self.commands[current];
        check!(cmd.reset());
        self.objects.clean_unused(&self.image_uniforms, current);
        check!(cmd.begin());
        cmd.bind_descriptor(
            self.image_uniforms.descriptor(),
            self.shader_layout.pipeline(),
        );
    }

    pub fn end_draw(&mut self) {
        if let RenderStage::Before = self.render_stage {
            panic!(error!("cannot end draw stage before draw stage"));
        } else {
            self.render_stage = RenderStage::Before;
        }

        let buffer = check!(self.commands[self.device.current_frame()].end());
        check!(self.device.submit(buffer));
        check!(self.device.present(&self.swapchain));
    }

    pub fn draw_on_window(&mut self, camera: &Camera, draw_callback: impl Fn(&mut Target<'_>)) {
        if let RenderStage::Before = self.render_stage {
            panic!(error!("cannot draw before draw stage"));
        }

        let mut target = check!(Target::new(&self.builtins, &self.objects));
        draw_callback(&mut target);

        let framebuffer = &self.window_framebuffers[self.swapchain.current()];
        let window_pass = self.render_passes.window();

        check!(self.forward_renderer.draw(ForwardDrawOptions {
            framebuffer,
            color_pass: window_pass,
            render_passes: &self.render_passes,
            shader_layout: &self.shader_layout,
            camera,
            objects: &self.objects,
            builtins: &self.builtins,
            cmd: &self.commands[self.device.current_frame()],
            target,
            time: self.start_time.elapsed().as_secs_f32(),
            blit: false,
        }));
    }

    pub fn draw(
        &self,
        framebuffer: &Id<Framebuffer>,
        camera: &Camera,
        draw_callback: impl Fn(&mut Target<'_>),
    ) {
        if let RenderStage::Before = self.render_stage {
            panic!(error!("cannot draw before draw stage"));
        }

        let mut target = check!(Target::new(&self.builtins, &self.objects));
        draw_callback(&mut target);

        self.objects.with_framebuffer(framebuffer.id_ref(), |f| {
            let color_pass = self.render_passes.color();

            check!(self.forward_renderer.draw(ForwardDrawOptions {
                framebuffer: f,
                color_pass,
                render_passes: &self.render_passes,
                shader_layout: &self.shader_layout,
                camera,
                objects: &self.objects,
                builtins: &self.builtins,
                cmd: &self.commands[self.device.current_frame()],
                target,
                time: self.start_time.elapsed().as_secs_f32(),
                blit: true,
            }));
        });
    }

    pub fn create_texture_rgba(&self, raw: &[u8], width: u32, height: u32) -> Id<Texture> {
        debug!("creating rgba texture");
        let texture = check!(Texture::from_raw_rgba(
            &self.device,
            raw,
            width,
            height,
            &self.image_uniforms
        ));
        self.objects.add_texture(texture)
    }

    pub fn create_texture_rgb(&self, raw: &[u8], width: u32, height: u32) -> Id<Texture> {
        debug!("creating rgb texture");
        let texture = check!(Texture::from_raw_rgb(
            &self.device,
            raw,
            width,
            height,
            &self.image_uniforms
        ));
        self.objects.add_texture(texture)
    }

    pub fn create_texture_from_file(&self, path: impl AsRef<Path>) -> Result<Id<Texture>> {
        let img = image::open(path.as_ref())?;
        let (width, height) = img.dimensions();
        let data = img.to_rgba().into_raw();
        Ok(self.create_texture_rgba(&data, width, height))
    }

    pub fn create_mesh(&self, options: MeshOptions<'_>) -> Id<Mesh> {
        debug!("creating mesh");
        let mesh = check!(Mesh::new(&self.device, options));
        self.objects.add_mesh(mesh)
    }

    pub fn create_material(&self, options: MaterialOptions) -> Id<Material> {
        debug!("creating material");
        let material = check!(Material::new(&self.device, &self.shader_layout, options));
        self.objects.add_material(material)
    }

    pub fn with_material<F, R>(&self, material: &Id<Material>, fun: F) -> Option<R>
    where
        F: FnOnce(&mut Material) -> R,
    {
        self.objects.with_material(material.id_ref(), fun)
    }

    pub fn create_framebuffer(&self, width: u32, height: u32) -> Id<Framebuffer> {
        debug!("creating framebuffer");
        let framebuffer = check!(Framebuffer::color(
            &self.device,
            &self.render_passes,
            &self.image_uniforms,
            &self.shader_layout,
            width,
            height,
        ));
        self.objects.add_framebuffer(framebuffer)
    }

    pub fn create_shader(&self, source: &[u8], options: ShaderOptions) -> Id<Shader> {
        debug!("creating shader");
        let render_pass = self.render_passes.color();
        let shader = check!(Shader::new(
            &self.device,
            render_pass,
            &self.shader_layout,
            source,
            options,
        ));
        self.objects.add_shader(shader)
    }

    pub fn create_shader_from_file(
        &self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Result<Id<Shader>> {
        let source = fs::read(path.as_ref())?;
        Ok(self.create_shader(&source, options))
    }

    pub fn create_shader_from_file_watch(
        &self,
        path: impl AsRef<Path>,
        options: ShaderOptions,
    ) -> Result<Id<Shader>> {
        let path_buf = path.as_ref().to_path_buf();
        let id = self.create_shader_from_file(&path_buf, options)?;

        // setup watcher
        let render_passes = self.render_passes.clone();
        let shader_layout = self.shader_layout.clone();
        let device = self.device.clone();
        let objects = self.objects.clone();
        let kill_recv = self.thread_kill.receiver();
        let id_ref = id.id_ref();

        thread::spawn(move || {
            let (sender, receiver) = channel::unbounded();
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

                            let source = check!(fs::read(&path_buf));
                            let color_pass = render_passes.color();
                            let shader = check!(Shader::new(
                                &device,
                                &color_pass,
                                &shader_layout,
                                &source,
                                options,
                            ));
                            objects.replace_shader(id_ref, shader, device.current_frame());
                        }
                    }
                }
            }
        });

        Ok(id)
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
        }
    }
}
