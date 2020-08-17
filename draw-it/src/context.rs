// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Context - draw-it application entrypoint

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Instant;

use crate::color::Color;
use crate::device::pick_gpu;
use crate::device::Device;
use crate::error::ErrorKind;
use crate::error::Result;
use crate::image::CoreFramebuffer;
use crate::image::CoreTexture;
use crate::image::Cubemap;
use crate::image::CubemapOptions;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::instance::Instance;
use crate::mesh::CoreMesh;
use crate::mesh::Mesh;
use crate::pipeline::CoreMaterial;
use crate::pipeline::CoreShader;
use crate::pipeline::ImageUniform;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::quality::Quality;
use crate::quality::QualityOptions;
use crate::renderer::Camera;
use crate::renderer::CameraType;
use crate::renderer::ForwardRenderer;
use crate::renderer::Target;
use crate::resource::Builtins;
use crate::resource::Index;
use crate::resource::ResourceManager;
use crate::stats::Stats;
use crate::surface::Surface;
use crate::surface::Swapchain;
use crate::surface::VSync;
use crate::surface::WindowHandle;
use crate::watch::watch_file;

#[cfg(feature = "ui")]
use crate::ui::Ui;
#[cfg(feature = "ui")]
use crate::ui::UiFrame;

#[cfg(feature = "window")]
use crate::window::Window;
#[cfg(feature = "window")]
use crate::window::WindowOptions;

const FPS_SAMPLE_COUNT: usize = 128;

pub struct Context {
    pub main_camera: Camera,
    pub builtins: Builtins,

    // Renderers
    forward_renderer: ForwardRenderer,

    // UI
    #[cfg(feature = "ui")]
    ui: Option<Ui>,

    // Resources
    resources: ResourceManager,
    skybox: Cubemap,

    // Vulkan
    window_framebuffers: Arc<Mutex<Vec<CoreFramebuffer>>>,
    image_uniform: ImageUniform,
    shader_layout: Arc<ShaderLayout>,
    swapchain: Swapchain,
    device: Arc<Device>,
    surface: Surface,
    gpu_index: usize,
    instance: Arc<Instance>,

    // Misc
    camera_type: CameraType,
    stats: Stats,
    render_stage: RenderStage,
    start_time: Instant,
    frame_time: Instant,
    frame_count: usize,
    fps_samples: [u32; FPS_SAMPLE_COUNT],
    msaa: Msaa,
    vsync: VSync,

    // Hot Reload
    hot_reload_sender: Sender<(Index, PathBuf)>,
    hot_reload_receiver: Receiver<(Index, PathBuf)>,

    // Window
    #[cfg(feature = "window")]
    glfw: Option<glfw::Glfw>,
    #[cfg(feature = "window")]
    event_receiver: Option<Receiver<(f64, glfw::WindowEvent)>>,
}

#[derive(Debug, Copy, Clone)]
pub struct ContextOptions {
    pub quality: Quality,
    pub vsync: VSync,
    pub camera: CameraType,
}

#[derive(Copy, Clone)]
enum RenderStage {
    Before,
    During,
}

impl Context {
    pub fn new(window: WindowHandle, options: ContextOptions) -> Result<Self> {
        let instance = Arc::new(Instance::new()?);
        let surface = Surface::new(&instance, window)?;

        let QualityOptions {
            anisotropy,
            msaa,
            pcf,
            shadow_map_size,
        } = options.quality.options();
        let vsync = options.vsync;

        // setup device stuff
        let mut gpu_properties_list = instance.gpu_properties(&surface)?;
        let gpu_index = pick_gpu(&gpu_properties_list, vsync, msaa)?;
        let gpu_properties = gpu_properties_list.remove(gpu_index);
        let device = Arc::new(Device::new(&instance, &gpu_properties, gpu_index)?);
        let swapchain = Swapchain::new(&device, &surface, &gpu_properties, vsync)?;

        info!("using anisotropy level {}", anisotropy);
        info!("using msaa level {:?}", msaa);
        info!("using vsync {:?}", vsync);

        // setup shader stuff
        let shader_layout = ShaderLayout::new(&device)?;
        let mut image_uniform = ImageUniform::new(&device, &shader_layout, anisotropy)?;

        // setup framebuffers
        let window_framebuffers = CoreFramebuffer::for_swapchain(
            &device,
            &swapchain,
            &shader_layout,
            options.camera,
            msaa,
        )?;

        // setup resources
        let mut resources = ResourceManager::new();
        let builtins = Builtins::new(
            &device,
            &mut resources,
            &window_framebuffers[0],
            &shader_layout,
            &mut image_uniform,
        )?;
        let mut skybox = Cubemap::new(
            &device,
            CubemapOptions {
                format: ImageFormat::Rgba,
                top: &[255, 255, 255, 255],
                bottom: &[255, 255, 255, 255],
                front: &[255, 255, 255, 255],
                back: &[255, 255, 255, 255],
                left: &[255, 255, 255, 255],
                right: &[255, 255, 255, 255],
                size: 1,
            },
        )?;
        image_uniform.set_skybox(skybox.add_view()?);

        // setup renderer
        let forward_renderer = ForwardRenderer::new(
            &device,
            &shader_layout,
            &mut image_uniform,
            shadow_map_size,
            pcf,
        )?;

        let main_camera = Camera::new(
            options.camera,
            window.width as f32,
            window.height as f32,
            100.0,
        );

        let (hot_reload_sender, hot_reload_receiver) = mpsc::channel();

        Ok(Self {
            window_framebuffers: Arc::new(Mutex::new(window_framebuffers)),
            shader_layout: Arc::new(shader_layout),
            fps_samples: [0; FPS_SAMPLE_COUNT],
            render_stage: RenderStage::Before,
            camera_type: options.camera,
            start_time: Instant::now(),
            frame_time: Instant::now(),
            stats: Default::default(),
            frame_count: 0,
            hot_reload_receiver,
            hot_reload_sender,
            forward_renderer,
            image_uniform,
            main_camera,
            resources,
            swapchain,
            gpu_index,
            builtins,
            instance,
            surface,
            skybox,
            device,
            msaa,
            vsync,

            #[cfg(feature = "ui")]
            ui: None,
            #[cfg(feature = "window")]
            glfw: None,
            #[cfg(feature = "window")]
            event_receiver: None,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.device.wait_for_idle()?;
        self.surface.resize(width, height);
        self.main_camera.width = width as f32;
        self.main_camera.height = height as f32;

        let gpu_properties = self
            .instance
            .gpu_properties(&self.surface)?
            .remove(self.gpu_index);
        self.swapchain
            .recreate(&self.surface, &gpu_properties, self.vsync)?;

        let mut framebuffers = self
            .window_framebuffers
            .lock()
            .expect("poisoned framebuffers");
        *framebuffers = CoreFramebuffer::for_swapchain(
            &self.device,
            &self.swapchain,
            &self.shader_layout,
            self.camera_type,
            self.msaa,
        )?;

        #[cfg(feature = "ui")]
        if let Some(ui) = &mut self.ui {
            ui.resize(&mut self.resources, &mut self.image_uniform, width, height);
        }

        Ok(())
    }

    pub fn draw_on_window(&mut self, draw_callback: impl Fn(&mut Target<'_>)) -> Result<()> {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw()?;
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins)?;
        draw_callback(&mut target);
        #[cfg(feature = "ui")]
        if let Some(ui) = &self.ui {
            if ui.drawn() {
                target.blit_framebuffer(ui.framebuffer());
            }
        }
        let render_data = target.render_data();

        // draw
        {
            let framebuffer = &mut self
                .window_framebuffers
                .lock()
                .expect("poisoned framebuffer")[self.swapchain.current()];
            framebuffer.camera = self.main_camera.clone();

            self.forward_renderer.draw_core(
                framebuffer,
                &mut self.resources,
                &self.builtins,
                &self.shader_layout,
                render_data,
                &mut self.stats,
            )?;
        }

        self.end_draw()?;
        Ok(())
    }

    pub fn draw(
        &mut self,
        framebuffer: &Framebuffer,
        draw_callback: impl Fn(&mut Target<'_>),
    ) -> Result<()> {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw()?;
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins)?;
        draw_callback(&mut target);
        let render_data = target.render_data();

        // draw
        self.forward_renderer.draw(
            &framebuffer.index,
            &mut self.resources,
            &self.builtins,
            &self.shader_layout,
            render_data,
            &mut self.stats,
        )?;

        Ok(())
    }

    pub fn create_texture(&mut self, pixels: &[Color], width: u32) -> Result<Texture> {
        let data = pixels
            .iter()
            .map(|p| vec![p.r, p.g, p.b, p.a])
            .flatten()
            .collect::<Vec<_>>();
        let (index, _) = self.resources.textures.add(CoreTexture::new(
            &self.device,
            &mut self.image_uniform,
            TextureOptions {
                format: ImageFormat::Rgba,
                height: pixels.len() as u32 / width,
                width,
                data,
            },
        )?);
        Ok(Texture::new(index))
    }

    pub fn set_skybox(&mut self, pixels: [&[u8]; 6], size: u32) -> Result<()> {
        let mut cubemap = Cubemap::new(
            &self.device,
            CubemapOptions {
                format: ImageFormat::Rgba,
                top: pixels[0],
                bottom: pixels[1],
                front: pixels[2],
                back: pixels[3],
                left: pixels[4],
                right: pixels[5],
                size,
            },
        )?;

        self.image_uniform.set_skybox(cubemap.add_view()?);
        self.skybox = cubemap;

        Ok(())
    }

    pub fn create_mesh(&mut self) -> Result<Mesh> {
        let (index, updater) = self.resources.meshes.add(CoreMesh::new(&self.device)?);
        Ok(Mesh::new(index, updater))
    }

    pub fn duplicate_mesh(&mut self, mesh: &Mesh) -> Result<Mesh> {
        let (index, updater) = self.resources.meshes.add(CoreMesh::new(&self.device)?);
        let mut result = Mesh::new(index, updater);
        result.vertices = mesh.vertices.clone();
        result.normals = mesh.normals.clone();
        result.colors = mesh.colors.clone();
        result.uvs = mesh.uvs.clone();
        result.indices = mesh.indices.clone();
        result.update();
        Ok(result)
    }

    pub fn combine_meshes(&mut self, meshes: &[Mesh]) -> Result<Mesh> {
        let (index, updater) = self.resources.meshes.add(CoreMesh::new(&self.device)?);
        Ok(Mesh::combine(index, updater, meshes))
    }

    pub fn create_material(&mut self) -> Result<Material> {
        let (index, updater) = self
            .resources
            .materials
            .add(CoreMaterial::new(&self.device, &self.shader_layout)?);
        Ok(Material::new(index, updater))
    }

    pub fn create_framebuffer(
        &mut self,
        t: CameraType,
        width: u32,
        height: u32,
    ) -> Result<Framebuffer> {
        let (index, updater) = self.resources.framebuffers.add(CoreFramebuffer::new(
            &self.device,
            &self.shader_layout,
            &mut self.image_uniform,
            FramebufferOptions {
                attachment_formats: &[ImageFormat::Sbgra],
                msaa: self.msaa,
                camera_type: t,
                depth: true,
                width,
                height,
            },
        )?);
        let mut framebuffer = Framebuffer::new(index, updater);
        framebuffer.width = width;
        framebuffer.height = height;
        Ok(framebuffer)
    }

    pub fn create_shader(&mut self, source: &[u8]) -> Result<Shader> {
        let framebuffer = &self
            .window_framebuffers
            .lock()
            .expect("poisoned framebuffers")[0];
        let (index, _) = self.resources.shaders.add(CoreShader::new(
            &self.device,
            framebuffer,
            &self.shader_layout,
            source,
        )?);
        Ok(Shader::new(index))
    }

    pub fn create_shader_from_file(&mut self, path: impl AsRef<Path>) -> Result<Shader> {
        let source = fs::read(path.as_ref())?;
        self.create_shader(&source)
    }

    pub fn stats(&self) -> Stats {
        self.stats
    }

    fn begin_draw(&mut self) -> Result<()> {
        self.render_stage = RenderStage::During;
        self.device.next_frame(&mut self.swapchain)?;
        self.resources.clean_unused(&mut self.image_uniform);
        self.resources.update_if_needed(&mut self.image_uniform);

        // hot-reload shaders
        for (index, path) in self.hot_reload_receiver.try_iter() {
            let source = fs::read(&path).expect("bad read");
            *self.resources.shaders.get_mut(&index) = CoreShader::new(
                &self.device,
                &self.window_framebuffers.lock().expect("bad lock")[0],
                &self.shader_layout,
                &source,
            )
            .expect("bad shader recreation");
            info!("shader {:?} was reloaded", path);
        }

        self.image_uniform.update_if_needed();
        self.device.cmd_bind_uniform(
            self.device.command_buffer(),
            &self.shader_layout,
            &self.image_uniform,
        );
        self.stats = Stats {
            time: self.start_time.elapsed().as_secs_f32(),
            ..Default::default()
        };

        #[cfg(feature = "ui")]
        if let Some(ui) = &mut self.ui {
            ui.reset();
        }

        Ok(())
    }

    fn end_draw(&mut self) -> Result<()> {
        self.render_stage = RenderStage::Before;
        self.device.submit()?;
        self.device.present(&self.swapchain)?;

        // update delta time
        let delta_time = self.frame_time.elapsed();
        self.stats.delta_time = delta_time.as_secs_f32();
        self.frame_time = Instant::now();
        self.fps_samples[self.frame_count % FPS_SAMPLE_COUNT] =
            1_000_000 / delta_time.as_micros() as u32;
        self.frame_count += 1;
        self.stats.fps =
            (self.fps_samples.iter().sum::<u32>() as f32 / FPS_SAMPLE_COUNT as f32).ceil() as u32;

        Ok(())
    }

    #[cfg(feature = "window")]
    pub fn with_window(
        c_options: ContextOptions,
        w_options: WindowOptions<'_>,
    ) -> Result<(Self, Window)> {
        use glfw::ClientApiHint;
        use glfw::WindowHint;
        use glfw::WindowMode;

        let WindowOptions {
            title,
            width,
            height,
            resizable,
        } = w_options;

        // create glfw window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        glfw.window_hint(WindowHint::Resizable(resizable));
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));

        let (mut window, event_receiver) = glfw
            .create_window(width, height, title, WindowMode::Windowed)
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
            width,
            height,
        };

        #[cfg(target_os = "linux")]
        let handle = WindowHandle {
            xlib_window: window.get_x11_window(),
            xlib_display: glfw.get_x11_display(),
            width,
            height,
        };

        #[cfg(target_os = "macos")]
        let handle = WindowHandle {
            ns_window: window.get_cocoa_window(),
            width,
            height,
        };

        let mut context = Self::new(handle, c_options)?;

        // attach glfw to context
        context.attach_glfw(glfw, event_receiver);

        // create ui renderer
        #[cfg(feature = "ui")]
        context.attach_ui(width, height)?;

        Ok((context, Window::new(window)))
    }

    #[cfg(feature = "window")]
    pub(crate) fn attach_glfw(
        &mut self,
        glfw: glfw::Glfw,
        event_receiver: Receiver<(f64, glfw::WindowEvent)>,
    ) {
        self.glfw = Some(glfw);
        self.event_receiver = Some(event_receiver);
    }

    #[cfg(feature = "ui")]
    pub(crate) fn attach_ui(&mut self, width: u32, height: u32) -> Result<()> {
        let ui = Ui::new(
            &self.device,
            &self.shader_layout,
            &mut self.image_uniform,
            &mut self.resources,
            width,
            height,
        )?;
        self.ui = Some(ui);

        Ok(())
    }

    #[cfg(feature = "window")]
    pub fn poll_events(&mut self, window: &mut Window) -> Result<()> {
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
                    self.resize(w as u32, h as u32)?;
                    window.handle_resize(w as u32, h as u32);
                    window.reset_resize();

                    info!("resized window to {}x{}", w, h);
                }
            }

            // pause if just resized
            polling = window.raw_size() == (0, 0) || window.last_resize().is_some();
        }

        Ok(())
    }

    pub fn create_shader_from_file_watch(&mut self, path: impl AsRef<Path>) -> Result<Shader> {
        let shader = self.create_shader_from_file(&path)?;
        watch_file(path, shader.index.clone(), self.hot_reload_sender.clone());
        Ok(shader)
    }

    #[cfg(feature = "image")]
    pub fn create_texture_from_file(&mut self, path: impl AsRef<Path>) -> Result<Texture> {
        use png::ColorType;
        use png::Decoder;
        use std::fs::File;

        let decoder = Decoder::new(File::open(path)?);
        let (info, mut reader) = decoder.read_info()?;

        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data)?;

        let format = match info.color_type {
            ColorType::RGBA => ImageFormat::Srgba,
            ColorType::RGB => ImageFormat::Srgb,
            ColorType::Grayscale => ImageFormat::Gray,
            _ => return Err(ErrorKind::UnsupportedFormat(format!("{:?}", info.color_type)).into()),
        };

        let (index, _) = self.resources.textures.add(CoreTexture::new(
            &self.device,
            &mut self.image_uniform,
            TextureOptions {
                width: info.width,
                height: info.height,
                format,
                data,
            },
        )?);
        Ok(Texture::new(index))
    }

    #[cfg(feature = "image")]
    pub fn set_skybox_from_file(&mut self, paths: [impl AsRef<Path>; 6]) -> Result<()> {
        use png::ColorType;
        use png::Decoder;
        use std::fs::File;

        use crate::image::with_alpha;

        let mut size = 0;
        let mut format = ImageFormat::Srgba;
        let mut data = vec![];
        for path in &paths {
            let decoder = Decoder::new(File::open(path)?);
            let (info, mut reader) = decoder.read_info()?;

            let mut buf = vec![0; info.buffer_size()];
            reader.next_frame(&mut buf)?;

            let f = match info.color_type {
                ColorType::RGBA | ColorType::RGB => ImageFormat::Srgba,
                ColorType::Grayscale => ImageFormat::Gray,
                _ => {
                    return Err(
                        ErrorKind::UnsupportedFormat(format!("{:?}", info.color_type)).into(),
                    )
                }
            };

            if let ColorType::RGB = info.color_type {
                buf = with_alpha(buf);
            }

            if f != format {
                return Err(ErrorKind::NonMatchingCubemapFormat(format!(
                    "{:?} ({:?})",
                    f,
                    path.as_ref()
                ))
                .into());
            }

            format = f;

            size = info.width;
            data.push(buf);
        }

        let mut cubemap = Cubemap::new(
            &self.device,
            CubemapOptions {
                top: &data[0],
                bottom: &data[1],
                front: &data[2],
                back: &data[3],
                left: &data[4],
                right: &data[5],
                format,
                size,
            },
        )?;

        self.image_uniform.set_skybox(cubemap.add_view()?);
        self.skybox = cubemap;

        Ok(())
    }

    #[cfg(feature = "ui")]
    pub fn draw_ui(&mut self, draw_fn: impl FnMut(&UiFrame<'_>)) -> Result<()> {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw()?;
        }

        self.ui.as_mut().ok_or(ErrorKind::UnitializedUi)?.draw(
            &self.shader_layout,
            &mut self.resources,
            &mut self.image_uniform,
            draw_fn,
        )?;

        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.device.wait_for_idle().expect("bad wait");
    }
}

impl Default for ContextOptions {
    fn default() -> Self {
        Self {
            quality: Quality::Medium,
            vsync: VSync::On,
            camera: CameraType::Perspective,
        }
    }
}
