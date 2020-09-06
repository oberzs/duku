// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Context - draw-it application entrypoint

use std::rc::Rc;
use std::time::Instant;

use crate::color::Color;
use crate::device::pick_gpu;
use crate::device::Device;
use crate::device::Stats;
use crate::error::Result;
use crate::image::CoreFramebuffer;
use crate::image::CoreTexture;
use crate::image::Cubemap;
use crate::image::CubemapSides;
use crate::image::Framebuffer;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::image::Size;
use crate::image::Texture;
use crate::instance::Instance;
use crate::mesh::CoreMesh;
use crate::mesh::Mesh;
use crate::mesh::MeshBuilder;
use crate::pipeline::CoreMaterial;
use crate::pipeline::CoreShader;
use crate::pipeline::Material;
use crate::pipeline::MaterialBuilder;
use crate::pipeline::Shader;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;
use crate::quality::Quality;
use crate::quality::QualitySettings;
use crate::renderer::Camera;
use crate::renderer::ForwardRenderer;
use crate::renderer::Target;
use crate::storage::Builtins;
use crate::storage::Storage;
use crate::surface::Surface;
use crate::surface::Swapchain;
use crate::surface::VSync;
use crate::surface::WindowHandle;

#[cfg(feature = "ui")]
use crate::ui::Ui;
#[cfg(feature = "ui")]
use crate::ui::UiFrame;

#[cfg(feature = "window")]
use crate::window::Window;

const FPS_SAMPLE_COUNT: usize = 64;

pub struct Context {
    // Renderers
    forward_renderer: ForwardRenderer,

    // UI
    #[cfg(feature = "ui")]
    ui: Option<Ui>,

    // Resources
    storage: Storage,
    skybox: Cubemap,
    builtins: Builtins,

    // Vulkan
    window_framebuffers: Vec<CoreFramebuffer>,
    shader_images: ShaderImages,
    shader_layout: ShaderLayout,
    swapchain: Swapchain,
    device: Rc<Device>,
    surface: Surface,
    gpu_index: usize,
    instance: Instance,

    // Misc
    render_stage: RenderStage,
    frame_time: Instant,
    frame_count: usize,
    fps_samples: [u32; FPS_SAMPLE_COUNT],
    fps: u32,
    delta_time: f32,
    msaa: Msaa,
    vsync: VSync,

    // Hot Reload
    #[cfg(feature = "glsl")]
    hot_reload_sender: std::sync::mpsc::Sender<(u32, std::path::PathBuf)>,
    #[cfg(feature = "glsl")]
    hot_reload_receiver: std::sync::mpsc::Receiver<(u32, std::path::PathBuf)>,

    // Window
    #[cfg(feature = "window")]
    glfw: Option<glfw::Glfw>,
    #[cfg(feature = "window")]
    event_receiver: Option<std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>>,
}

#[derive(Debug, Clone)]
pub struct ContextBuilder {
    quality: Quality,
    vsync: VSync,
    window: Option<WindowHandle>,
}

#[cfg(feature = "window")]
#[derive(Debug, Clone)]
pub struct WindowBuilder {
    context: ContextBuilder,
    title: String,
    resizable: bool,
    width: u32,
    height: u32,
}

#[derive(Copy, Clone)]
enum RenderStage {
    Before,
    During,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder::default()
    }

    fn new(window: WindowHandle, quality: Quality, vsync: VSync) -> Result<Self> {
        let instance = Instance::new();
        let surface = Surface::new(&instance, window);

        let QualitySettings {
            anisotropy,
            msaa,
            pcf,
            shadow_map_size,
        } = quality.settings();

        // setup device stuff
        let mut gpu_properties_list = instance.gpu_properties(&surface);
        let gpu_index = pick_gpu(&gpu_properties_list, vsync, msaa)?;
        let gpu_properties = gpu_properties_list.remove(gpu_index);
        let device = Rc::new(Device::new(&instance, &gpu_properties, gpu_index));
        let swapchain = Swapchain::new(&device, &surface, &gpu_properties, vsync);

        info!("using anisotropy level {}", anisotropy);
        info!("using msaa level {:?}", msaa);
        info!("using vsync {:?}", vsync);

        // setup shader stuff
        let shader_layout = ShaderLayout::new(&device);
        let mut shader_images = ShaderImages::new(&device, &shader_layout, anisotropy);

        // setup framebuffers
        let window_framebuffers =
            CoreFramebuffer::for_swapchain(&device, &swapchain, &shader_layout, msaa);

        // setup storage
        let mut storage = Storage::new();
        let builtins = Builtins::new(
            &device,
            &mut storage,
            &window_framebuffers[0],
            &shader_layout,
            &mut shader_images,
        )?;
        let mut skybox = Cubemap::new(
            &device,
            1,
            ImageFormat::Rgba,
            CubemapSides {
                top: vec![255, 255, 255, 255],
                bottom: vec![255, 255, 255, 255],
                front: vec![255, 255, 255, 255],
                back: vec![255, 255, 255, 255],
                left: vec![255, 255, 255, 255],
                right: vec![255, 255, 255, 255],
            },
        );
        shader_images.set_skybox(skybox.add_view(&device));

        // setup renderer
        let forward_renderer = ForwardRenderer::new(
            &device,
            &shader_layout,
            &mut shader_images,
            shadow_map_size,
            pcf,
        );

        #[cfg(feature = "glsl")]
        let (hot_reload_sender, hot_reload_receiver) = std::sync::mpsc::channel();

        Ok(Self {
            fps_samples: [0; FPS_SAMPLE_COUNT],
            render_stage: RenderStage::Before,
            frame_time: Instant::now(),
            fps: 0,
            delta_time: 0.0,
            frame_count: 0,
            window_framebuffers,
            forward_renderer,
            shader_images,
            shader_layout,
            storage,
            swapchain,
            gpu_index,
            builtins,
            instance,
            surface,
            skybox,
            device,
            msaa,
            vsync,

            #[cfg(feature = "glsl")]
            hot_reload_sender,
            #[cfg(feature = "glsl")]
            hot_reload_receiver,
            #[cfg(feature = "ui")]
            ui: None,
            #[cfg(feature = "window")]
            glfw: None,
            #[cfg(feature = "window")]
            event_receiver: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.device.wait_idle();
        self.surface.resize(width, height);

        let gpu_properties = self
            .instance
            .gpu_properties(&self.surface)
            .remove(self.gpu_index);
        self.swapchain
            .recreate(&self.device, &self.surface, &gpu_properties, self.vsync);

        self.window_framebuffers = CoreFramebuffer::for_swapchain(
            &self.device,
            &self.swapchain,
            &self.shader_layout,
            self.msaa,
        );

        #[cfg(feature = "ui")]
        if let Some(ui) = &mut self.ui {
            ui.resize(
                &mut self.storage,
                &mut self.shader_images,
                Size::new(width, height),
            );
        }
    }

    pub fn draw_on_window(
        &mut self,
        camera: Option<&Camera>,
        draw_callback: impl Fn(&mut Target<'_>),
    ) {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins);
        draw_callback(&mut target);
        #[cfg(feature = "ui")]
        if let Some(ui) = &self.ui {
            if ui.drawn() {
                // target.blit_framebuffer(ui.framebuffer());
            }
        }

        let framebuffer = &self.window_framebuffers[self.swapchain.current()];

        let cam = get_camera(camera, framebuffer.size());

        // draw
        self.forward_renderer.draw(
            framebuffer,
            &cam,
            &self.storage,
            &self.shader_layout,
            target,
        );

        self.end_draw();
    }

    pub fn draw(
        &mut self,
        framebuffer: &Framebuffer,
        camera: Option<&Camera>,
        draw_callback: impl Fn(&mut Target<'_>),
    ) {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins);
        draw_callback(&mut target);

        let cam = get_camera(camera, Size::new(framebuffer.width, framebuffer.height));

        // draw
        self.forward_renderer.draw(
            self.storage.framebuffers.get(&framebuffer.index),
            &cam,
            &self.storage,
            &self.shader_layout,
            target,
        );
    }

    pub fn create_texture(&mut self, pixels: &[Color], width: u32, height: u32) -> Texture {
        let data = pixels
            .iter()
            .map(|p| vec![p.r, p.g, p.b, p.a])
            .flatten()
            .collect::<Vec<_>>();
        let tex = CoreTexture::new(
            &self.device,
            &mut self.shader_images,
            data,
            Size::new(width, height),
            ImageFormat::Rgba,
        );
        let shader_index = tex.shader_index();
        let (index, _) = self.storage.textures.add(tex);
        Texture::new(index, shader_index)
    }

    pub fn create_mesh(&mut self) -> Mesh {
        let (index, updater) = self.storage.meshes.add(CoreMesh::new(&self.device));
        Mesh::new(index, updater)
    }

    pub fn build_mesh(&mut self) -> MeshBuilder {
        MeshBuilder::new(self.create_mesh())
    }

    pub fn duplicate_mesh(&mut self, mesh: &Mesh) -> Mesh {
        let (index, updater) = self.storage.meshes.add(CoreMesh::new(&self.device));
        let mut result = Mesh::new(index, updater);
        result.vertices = mesh.vertices.clone();
        result.normals = mesh.normals.clone();
        result.colors = mesh.colors.clone();
        result.uvs = mesh.uvs.clone();
        result.indices = mesh.indices.clone();
        result.update();
        result
    }

    pub fn combine_meshes(&mut self, meshes: &[Mesh]) -> Mesh {
        let (index, updater) = self.storage.meshes.add(CoreMesh::new(&self.device));
        Mesh::combine(index, updater, meshes)
    }

    pub fn create_material(&mut self) -> Material {
        let (index, updater) = self
            .storage
            .materials
            .add(CoreMaterial::new(&self.device, &self.shader_layout));
        Material::new(index, updater)
    }

    pub fn build_material(&mut self) -> MaterialBuilder {
        MaterialBuilder::new(self.create_material())
    }

    pub fn create_framebuffer(&mut self, width: u32, height: u32) -> Framebuffer {
        let (index, updater) = self.storage.framebuffers.add(CoreFramebuffer::new(
            &self.device,
            &self.shader_layout,
            &mut self.shader_images,
            &[ImageFormat::Depth, ImageFormat::Sbgra],
            self.msaa,
            Size::new(width, height),
        ));
        let mut framebuffer = Framebuffer::new(index, updater);
        framebuffer.width = width;
        framebuffer.height = height;
        framebuffer
    }

    pub fn create_shader_spirv(&mut self, source: &[u8]) -> Result<Shader> {
        let (index, _) = self.storage.shaders.add(CoreShader::from_spirv_bytes(
            &self.device,
            &self.window_framebuffers[0],
            &self.shader_layout,
            source,
        )?);
        Ok(Shader::new(index))
    }

    pub fn stats(&self) -> Stats {
        self.device.stats()
    }

    pub const fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub const fn fps(&self) -> u32 {
        self.fps
    }

    fn begin_draw(&mut self) {
        self.render_stage = RenderStage::During;
        self.device.next_frame(&mut self.swapchain);
        self.storage
            .clear_unused(&self.device, &mut self.shader_images);
        self.storage
            .update_if_needed(&self.device, &mut self.shader_images);

        // hot-reload shaders
        #[cfg(feature = "glsl")]
        for (pointer, path) in self.hot_reload_receiver.try_iter() {
            let source = std::fs::read_to_string(&path).expect("bad read");

            match CoreShader::from_glsl_string(
                &self.device,
                &self.window_framebuffers[0],
                &self.shader_layout,
                source,
            ) {
                Ok(new_shader) => {
                    *self
                        .storage
                        .shaders
                        .get_mut(&crate::storage::Index::new(pointer)) = new_shader;
                    info!("shader {:?} was reloaded", path);
                }
                Err(err) => warn!("{}", err),
            }
        }

        self.shader_images.update_if_needed(&self.device);
        self.device
            .commands()
            .bind_descriptor(&self.shader_layout, self.shader_images.descriptor());

        #[cfg(feature = "ui")]
        if let Some(ui) = &mut self.ui {
            ui.reset();
        }
    }

    fn end_draw(&mut self) {
        self.render_stage = RenderStage::Before;
        self.device.submit();
        self.device.present(&self.swapchain);

        // update delta time
        let delta_time = self.frame_time.elapsed();
        self.delta_time = delta_time.as_secs_f32();
        self.frame_time = Instant::now();
        self.fps_samples[self.frame_count % FPS_SAMPLE_COUNT] =
            1_000_000 / delta_time.as_micros() as u32;
        self.frame_count += 1;
        self.fps =
            (self.fps_samples.iter().sum::<u32>() as f32 / FPS_SAMPLE_COUNT as f32).ceil() as u32;
    }

    #[cfg(feature = "window")]
    pub(crate) fn attach_glfw(
        &mut self,
        glfw: glfw::Glfw,
        event_receiver: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    ) {
        self.glfw = Some(glfw);
        self.event_receiver = Some(event_receiver);
    }

    #[cfg(feature = "ui")]
    pub(crate) fn attach_ui(&mut self, width: u32, height: u32) {
        self.ui = Some(Ui::new(
            &self.device,
            &self.shader_layout,
            &mut self.shader_images,
            &mut self.storage,
            Size::new(width, height),
        ));
    }

    #[cfg(feature = "window")]
    pub fn poll_events(&mut self, window: &mut Window) {
        use glfw::WindowEvent;
        use std::time::Duration;

        // clear events
        window.clear();

        // poll events
        let mut polling = true;
        while polling {
            self.glfw.as_mut().expect("bad glfw").poll_events();
            let receiver = self.event_receiver.as_ref().expect("bad event receiver");
            for (_, event) in glfw::flush_messages(receiver) {
                // update imgui
                #[cfg(feature = "ui")]
                if let Some(ui) = &mut self.ui {
                    ui.handle_event(&event);
                }

                // update window events
                match event {
                    WindowEvent::Key(key, _, action, _) => window.handle_key(key, action),
                    WindowEvent::CursorPos(x, y) => window.handle_mouse(x, y),
                    WindowEvent::Scroll(x, y) => window.handle_scroll(x, y),
                    WindowEvent::Size(w, h) if w != 0 && h != 0 => window.record_resize(),
                    WindowEvent::MouseButton(button, action, _) => {
                        window.handle_mouse_button(button, action)
                    }
                    _ => (),
                }
            }

            // check resize timing
            if let Some(last) = window.last_resize() {
                if Instant::now().duration_since(last) >= Duration::from_millis(100) {
                    let (w, h) = window.raw_size();
                    self.resize(w as u32, h as u32);
                    window.handle_resize(w as u32, h as u32);
                    window.reset_resize();

                    info!("resized window to {}x{}", w, h);
                }
            }

            // pause if just resized
            polling = window.raw_size() == (0, 0) || window.last_resize().is_some();
        }
    }

    #[cfg(feature = "glsl")]
    pub fn create_shader_glsl(
        &mut self,
        path: impl AsRef<std::path::Path>,
        watch: bool,
    ) -> Result<Shader> {
        use crate::watch::watch_file;

        let source = std::fs::read_to_string(&path)?;
        let (index, _) = self.storage.shaders.add(CoreShader::from_glsl_string(
            &self.device,
            &self.window_framebuffers[0],
            &self.shader_layout,
            source,
        )?);

        if watch {
            watch_file(path, index.pointer(), self.hot_reload_sender.clone());
        }

        Ok(Shader::new(index))
    }

    #[cfg(feature = "png")]
    pub fn create_texture_png_bytes(&mut self, bytes: Vec<u8>) -> Result<Texture> {
        let tex = CoreTexture::from_png_bytes(&self.device, &mut self.shader_images, bytes)?;
        let shader_index = tex.shader_index();
        let (index, _) = self.storage.textures.add(tex);
        Ok(Texture::new(index, shader_index))
    }

    #[cfg(feature = "png")]
    pub fn create_texture_png(&mut self, path: impl AsRef<std::path::Path>) -> Result<Texture> {
        let bytes = std::fs::read(path.as_ref())?;
        self.create_texture_png_bytes(bytes)
    }

    #[cfg(feature = "png")]
    pub fn set_skybox_png(
        &mut self,
        sides: CubemapSides<impl AsRef<std::path::Path>>,
    ) -> Result<()> {
        let mut cubemap = Cubemap::from_png_bytes(
            &self.device,
            CubemapSides {
                top: std::fs::read(sides.top)?,
                bottom: std::fs::read(sides.bottom)?,
                front: std::fs::read(sides.front)?,
                back: std::fs::read(sides.back)?,
                left: std::fs::read(sides.left)?,
                right: std::fs::read(sides.right)?,
            },
        )?;
        self.shader_images
            .set_skybox(cubemap.add_view(&self.device));
        self.skybox = cubemap;
        Ok(())
    }

    #[cfg(feature = "ui")]
    pub fn draw_ui(&mut self, draw_fn: impl FnMut(&UiFrame<'_>)) -> Result<()> {
        use crate::error::Error;

        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        self.ui.as_mut().ok_or(Error::UnitializedUi)?.draw(
            &self.shader_layout,
            &mut self.storage,
            draw_fn,
        );
        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.device.wait_idle();
        self.skybox.destroy(&self.device);
        self.shader_images.destroy(&self.device);
        self.storage.clear(&self.device, &mut self.shader_images);
        self.forward_renderer.destroy(&self.device);
        self.shader_layout.destroy(&self.device);
        self.device.destroy_swapchain(&self.swapchain);
        self.instance.destroy_surface(&self.surface);
    }
}

impl ContextBuilder {
    pub const fn vsync(mut self, vsync: VSync) -> Self {
        self.vsync = vsync;
        self
    }

    pub const fn no_vsync(mut self) -> Self {
        self.vsync = VSync::Off;
        self
    }

    pub const fn quality(mut self, quality: Quality) -> Self {
        self.quality = quality;
        self
    }

    pub const fn low_quality(mut self) -> Self {
        self.quality = Quality::Low;
        self
    }

    pub const fn medium_quality(mut self) -> Self {
        self.quality = Quality::Medium;
        self
    }

    pub const fn high_quality(mut self) -> Self {
        self.quality = Quality::High;
        self
    }

    pub const fn attach_window(mut self, window: WindowHandle) -> Self {
        self.window = Some(window);
        self
    }

    pub fn build(self) -> Result<Context> {
        let window = match self.window {
            Some(w) => w,
            None => unimplemented!(),
        };
        Context::new(window, self.quality, self.vsync)
    }

    #[cfg(feature = "window")]
    pub fn build_window(self, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            context: self,
            title: "".to_string(),
            resizable: false,
            width,
            height,
        }
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self {
            window: None,
            quality: Quality::Medium,
            vsync: VSync::On,
        }
    }
}

#[cfg(feature = "window")]
impl WindowBuilder {
    pub const fn resizable(mut self) -> Self {
        self.resizable = true;
        self
    }

    pub fn title<S: AsRef<str>>(mut self, title: S) -> Self {
        self.title = title.as_ref().to_string();
        self
    }

    pub fn build(self) -> Result<(Context, Window)> {
        use glfw::ClientApiHint;
        use glfw::WindowHint;
        use glfw::WindowMode;

        use crate::error::Error;

        // create glfw window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).map_err(|_| Error::InternalGlfw)?;

        glfw.window_hint(WindowHint::Resizable(self.resizable));
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

        let (mut window, event_receiver) = glfw
            .create_window(self.width, self.height, &self.title, WindowMode::Windowed)
            .expect("bad window");

        window.set_key_polling(true);
        window.set_scroll_polling(true);
        window.set_size_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_char_polling(true);

        // create context
        #[cfg(target_os = "windows")]
        let handle = WindowHandle {
            hwnd: window.get_win32_window(),
            width: self.width,
            height: self.height,
        };

        #[cfg(target_os = "linux")]
        let handle = WindowHandle {
            xlib_window: window.get_x11_window(),
            xlib_display: glfw.get_x11_display(),
            width: self.width,
            height: self.height,
        };

        #[cfg(target_os = "macos")]
        let handle = WindowHandle {
            ns_window: window.get_cocoa_window(),
            width: self.width,
            height: self.height,
        };

        let mut context = Context::new(handle, self.context.quality, self.context.vsync)?;

        // attach glfw to context
        context.attach_glfw(glfw, event_receiver);

        // create ui renderer
        #[cfg(feature = "ui")]
        context.attach_ui(self.width, self.height);

        Ok((context, Window::new(window)))
    }
}

fn get_camera(camera: Option<&Camera>, size: Size) -> Camera {
    match camera {
        Some(c) => {
            if c.autosize {
                let mut cam = Camera::new(
                    c.projection,
                    size.width as f32,
                    size.height as f32,
                    c.depth,
                    c.fov,
                );
                cam.transform = c.transform;
                cam
            } else {
                c.clone()
            }
        }
        // create default camera if not supplied
        None => Camera::orthographic(size.width as f32, size.height as f32),
    }
}
