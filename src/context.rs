// Oliver Berzs
// https://github.com/oberzs/draw-it

// Context - draw-it application entrypoint

use std::time::Instant;

use crate::color::Color;
use crate::device::pick_gpu;
use crate::device::Device;
use crate::device::Stats;
use crate::error::Result;
use crate::image::Cubemap;
use crate::image::CubemapSides;
use crate::image::Framebuffer;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::image::Size;
use crate::image::Texture;
use crate::instance::Instance;
use crate::mesh::Mesh;
use crate::mesh::MeshBuilder;
use crate::pipeline::Material;
use crate::pipeline::MaterialBuilder;
use crate::pipeline::Shader;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;
use crate::quality::Quality;
use crate::quality::QualitySettings;
use crate::renderer::Camera;
use crate::renderer::ForwardRenderer;
use crate::renderer::RenderStores;
use crate::renderer::Target;
use crate::storage;
use crate::storage::Builtins;
use crate::storage::Handle;
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
    // Vulkan
    instance: Instance,
    device: Device,
    gpu_index: usize,
    surface: Surface,
    swapchain: Swapchain,
    shader_layout: ShaderLayout,
    window_framebuffers: Vec<Framebuffer>,
    shader_images: ShaderImages,

    // Resources
    storage: Storage,
    skybox: Cubemap,
    builtins: Builtins,

    // Renderers
    forward_renderer: ForwardRenderer,

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
    hot_reload_sender: std::sync::mpsc::Sender<(Handle<Shader>, std::path::PathBuf)>,
    #[cfg(feature = "glsl")]
    hot_reload_receiver: std::sync::mpsc::Receiver<(Handle<Shader>, std::path::PathBuf)>,

    // Window
    #[cfg(feature = "window")]
    glfw: Option<glfw::Glfw>,
    #[cfg(feature = "window")]
    event_receiver: Option<std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>>,

    // UI
    #[cfg(feature = "ui")]
    ui: Option<Ui>,
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
        let device = Device::new(&instance, &gpu_properties, gpu_index);
        let swapchain = Swapchain::new(&device, &surface, &gpu_properties, vsync);

        info!("using anisotropy level {}", anisotropy);
        info!("using msaa level {:?}", msaa);
        info!("using vsync {:?}", vsync);

        // setup shader stuff
        let shader_layout = ShaderLayout::new(&device);
        let mut shader_images = ShaderImages::new(&device, &shader_layout, anisotropy);

        // setup framebuffers
        let window_framebuffers =
            Framebuffer::for_swapchain(&device, &swapchain, &shader_layout, msaa);

        // setup storage
        let mut storage = Storage::new();
        let builtins = Builtins::new(
            &device,
            &mut storage,
            &window_framebuffers[0],
            &shader_layout,
            &mut shader_images,
        );
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

        for framebuffer in &self.window_framebuffers {
            framebuffer.destroy(&self.device);
        }

        self.window_framebuffers = Framebuffer::for_swapchain(
            &self.device,
            &self.swapchain,
            &self.shader_layout,
            self.msaa,
        );

        #[cfg(feature = "ui")]
        if let Some(ui) = &mut self.ui {
            ui.resize(
                &self.device,
                &mut self.storage,
                &mut self.shader_images,
                Size::new(width, height),
            );
        }
    }

    #[allow(single_use_lifetimes)]
    pub fn draw_on_window<'target, F>(&mut self, camera: Option<&Camera>, draw_callback: F)
    where
        F: Fn(&mut Target<'target, '_>),
    {
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

        let framebuffer = &mut self.window_framebuffers[self.swapchain.current()];

        let cam = get_camera(camera, framebuffer.size());

        let stores = RenderStores {
            shaders: &self.storage.shaders,
            fonts: &self.storage.fonts,
            materials: &self.storage.materials,
            textures: &self.storage.textures,
            meshes: &self.storage.meshes,
        };

        // draw
        self.forward_renderer.draw(
            &self.device,
            framebuffer,
            &cam,
            stores,
            &self.shader_layout,
            target,
        );

        self.end_draw();
    }

    #[allow(single_use_lifetimes)]
    pub fn draw<'target, F>(
        &mut self,
        framebuffer: &Handle<Framebuffer>,
        camera: Option<&Camera>,
        draw_callback: F,
    ) where
        F: Fn(&mut Target<'target, '_>),
    {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins);
        draw_callback(&mut target);

        let frame = self.storage.framebuffers.get_mut(framebuffer);
        let cam = get_camera(camera, frame.size());

        let stores = RenderStores {
            shaders: &self.storage.shaders,
            fonts: &self.storage.fonts,
            materials: &self.storage.materials,
            textures: &self.storage.textures,
            meshes: &self.storage.meshes,
        };

        // draw
        self.forward_renderer.draw(
            &self.device,
            frame,
            &cam,
            stores,
            &self.shader_layout,
            target,
        );
    }

    pub fn create_texture(&mut self, pixels: &[Color], width: u32, height: u32) -> Handle<Texture> {
        let data = pixels
            .iter()
            .map(|p| vec![p.r, p.g, p.b, p.a])
            .flatten()
            .collect::<Vec<_>>();
        let tex = Texture::new(
            &self.device,
            &mut self.shader_images,
            data,
            Size::new(width, height),
            ImageFormat::Rgba,
        );
        self.storage.add_texture(tex)
    }

    pub fn create_mesh(&mut self) -> Handle<Mesh> {
        let mesh = Mesh::new(&self.device);
        self.storage.add_mesh(mesh)
    }

    pub fn create_mesh_cube(&mut self) -> Handle<Mesh> {
        self.storage.add_mesh(storage::create_cube(&self.device))
    }

    pub fn create_mesh_sphere_ico(&mut self, detail: u32) -> Handle<Mesh> {
        self.storage
            .add_mesh(storage::create_ico_sphere(&self.device, detail))
    }

    pub fn create_mesh_sphere_uv(&mut self, meridians: u32, parallels: u32) -> Handle<Mesh> {
        self.storage.add_mesh(storage::create_uv_sphere(
            &self.device,
            meridians,
            parallels,
        ))
    }

    pub fn mesh(&self, mesh: &Handle<Mesh>) -> &Mesh {
        self.storage.meshes.get(mesh)
    }

    pub fn mesh_mut(&mut self, mesh: &Handle<Mesh>) -> &mut Mesh {
        self.storage.meshes.get_mut(mesh)
    }

    pub fn build_mesh(&mut self) -> MeshBuilder<'_> {
        MeshBuilder {
            storage: &mut self.storage,
            mesh: Mesh::new(&self.device),
        }
    }

    pub fn duplicate_mesh(&mut self, mesh: &Handle<Mesh>) -> Handle<Mesh> {
        let m = self.storage.meshes.get(mesh);
        let mut result = Mesh::new(&self.device);
        result.set_vertices(m.vertices().to_vec());
        result.set_normals(m.normals().to_vec());
        result.set_colors(m.colors().to_vec());
        result.set_uvs(m.uvs().to_vec());
        result.set_indices(m.indices().to_vec());
        self.storage.add_mesh(result)
    }

    pub fn combine_meshes(&mut self, meshes: &[Handle<Mesh>]) -> Handle<Mesh> {
        let ms: Vec<_> = meshes.iter().map(|m| self.storage.meshes.get(m)).collect();
        let mesh = Mesh::combine(&self.device, &ms);
        self.storage.add_mesh(mesh)
    }

    pub fn create_material(&mut self) -> Handle<Material> {
        let mat = Material::new(&self.device, &self.shader_layout);
        self.storage.add_material(mat)
    }

    pub fn material(&self, material: &Handle<Material>) -> &Material {
        self.storage.materials.get(material)
    }

    pub fn material_mut(&mut self, material: &Handle<Material>) -> &mut Material {
        self.storage.materials.get_mut(material)
    }

    pub fn build_material(&mut self) -> MaterialBuilder<'_> {
        MaterialBuilder {
            storage: &mut self.storage,
            material: Material::new(&self.device, &self.shader_layout),
        }
    }

    pub fn create_framebuffer(&mut self, width: u32, height: u32) -> Handle<Framebuffer> {
        let framebuffer = Framebuffer::new(
            &self.device,
            &self.shader_layout,
            &mut self.shader_images,
            &[ImageFormat::Depth, ImageFormat::Sbgra],
            self.msaa,
            Size::new(width, height),
        );
        self.storage.add_framebuffer(framebuffer)
    }

    pub fn framebuffer_mut(&mut self, framebuffer: &Handle<Framebuffer>) -> &mut Framebuffer {
        self.storage.framebuffers.get_mut(framebuffer)
    }

    pub fn create_shader_spirv(&mut self, source: &[u8]) -> Result<Handle<Shader>> {
        let shader = Shader::from_spirv_bytes(
            &self.device,
            &self.window_framebuffers[0],
            &self.shader_layout,
            source,
        )?;
        Ok(self.storage.add_shader(shader))
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
        for (handle, path) in self.hot_reload_receiver.try_iter() {
            let source = std::fs::read_to_string(&path).expect("bad read");

            match Shader::from_glsl_string(
                &self.device,
                &self.window_framebuffers[0],
                &self.shader_layout,
                source,
            ) {
                Ok(new_shader) => {
                    *self.storage.shaders.get_mut(&handle) = new_shader;
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
    ) -> Result<Handle<Shader>> {
        use crate::watch::watch_file;

        let source = std::fs::read_to_string(&path)?;
        let handle = self.storage.add_shader(Shader::from_glsl_string(
            &self.device,
            &self.window_framebuffers[0],
            &self.shader_layout,
            source,
        )?);

        if watch {
            watch_file(path, handle.clone(), self.hot_reload_sender.clone());
        }

        Ok(handle)
    }

    #[cfg(feature = "png")]
    pub fn create_texture_png_bytes(&mut self, bytes: Vec<u8>) -> Result<Handle<Texture>> {
        let tex = Texture::from_png_bytes(&self.device, &mut self.shader_images, bytes, false)?;
        Ok(self.storage.add_texture(tex))
    }

    #[cfg(feature = "png")]
    pub fn create_texture_png(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Handle<Texture>> {
        let bytes = std::fs::read(path.as_ref())?;
        self.create_texture_png_bytes(bytes)
    }

    #[cfg(feature = "png")]
    pub fn create_texture_png_bytes_linear(&mut self, bytes: Vec<u8>) -> Result<Handle<Texture>> {
        let tex = Texture::from_png_bytes(&self.device, &mut self.shader_images, bytes, true)?;
        Ok(self.storage.add_texture(tex))
    }

    #[cfg(feature = "png")]
    pub fn create_texture_png_linear(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Handle<Texture>> {
        let bytes = std::fs::read(path.as_ref())?;
        self.create_texture_png_bytes_linear(bytes)
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
        self.skybox.destroy(&self.device);
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
            &self.device,
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
        #[cfg(feature = "ui")]
        if let Some(ui) = &self.ui {
            ui.destroy(&self.device);
        }
        self.shader_layout.destroy(&self.device);
        for framebuffer in &self.window_framebuffers {
            framebuffer.destroy(&self.device);
        }
        self.device.destroy_swapchain(&self.swapchain);
        self.instance.destroy_surface(&self.surface);

        self.device.destroy();
        self.instance.destroy();
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
