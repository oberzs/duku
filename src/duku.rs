// Oliver Berzs
// https://github.com/oberzs/duku

use std::convert::TryInto;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::device::pick_gpu;
use crate::device::Device;
use crate::device::Stats;
use crate::error::Result;
use crate::font::Font;
use crate::font::FontData;
use crate::image::Canvas;
use crate::image::Cubemap;
use crate::image::CubemapSides;
use crate::image::Format;
use crate::image::Mips;
use crate::image::Msaa;
use crate::image::Texture;
use crate::instance::Instance;
use crate::mesh::Mesh;
use crate::mesh::Model;
use crate::pipeline::Material;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConfig;
use crate::pipeline::Uniforms;
use crate::renderer::Camera;
use crate::renderer::ForwardRenderer;
use crate::renderer::Projection;
use crate::renderer::Target;
use crate::resources;
use crate::resources::Builtins;
use crate::resources::Handle;
use crate::resources::Resources;
use crate::surface::Surface;
use crate::surface::Swapchain;
use crate::surface::VSync;
use crate::surface::WindowHandle;

const FPS_SAMPLE_COUNT: usize = 64;

/// The renderer context.
///
/// Entrypoint into the duku API
///
/// # Examples
///
/// ```no_run
/// # use duku::Duku;
/// let (mut duku, window) = Duku::windowed(500, 500).unwrap();
/// ```
pub struct Duku {
    // Vulkan
    instance: Instance,
    device: Device,
    gpu_index: usize,
    uniforms: Uniforms,

    // Window resources
    window_canvases: Vec<Canvas>,
    swapchain: Option<Swapchain>,
    surface: Option<Surface>,

    // Resources
    resources: Resources,
    /// Built-in resources
    pub builtins: Builtins,

    // Renderers
    forward_renderer: ForwardRenderer,
    draw_calls: Vec<DrawCall>,

    // Misc
    render_stage: RenderStage,
    frame_time: Instant,
    frame_count: usize,
    fps_samples: [u32; FPS_SAMPLE_COUNT],
    fps: u32,
    delta_time: f32,
    msaa: Msaa,
    vsync: VSync,
}

/// The render context builder.
#[derive(Debug, Clone)]
pub struct DukuBuilder {
    shadow_map_size: u32,
    anisotropy: f32,
    msaa: Msaa,
    vsync: VSync,
    window: Option<WindowHandle>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RenderStage {
    Before,
    During,
}

struct DrawCall {
    target: Target,
    camera: Option<Camera>,
    canvas: Option<Handle<Canvas>>,
}

impl Duku {
    /// Create builder for duku context
    pub const fn builder() -> DukuBuilder {
        DukuBuilder {
            shadow_map_size: 2048,
            anisotropy: 4.0,
            msaa: Msaa::X4,
            vsync: VSync::On,
            window: None,
        }
    }

    /// Create headless duku context
    pub fn headless() -> Self {
        Self::builder().build()
    }

    /// Draw on the window canvas
    ///
    /// If `camera` is `None` a default camera that fits the
    /// canvas will be used.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use duku::Duku;
    /// # let (mut duku, _) = Duku::windowed(1, 1).unwrap();
    /// duku.draw(None, |t| {
    ///     // record drawing commands
    /// });
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if drawing hasn't begun or duku is in headless mode or if
    /// `begin` wasn't called yet.
    pub fn draw(&mut self, camera: Option<&Camera>, draw_fn: impl Fn(&mut Target)) {
        if self.render_stage == RenderStage::Before {
            panic!("cannot draw before calling 'begin'");
        }
        if self.swapchain.is_none() {
            panic!("cannot draw in headless mode");
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins);
        draw_fn(&mut target);
        //  let canvas = &self.window_canvases[swapchain.current()];
        // let cam = get_camera(camera, canvas.width, canvas.height);

        self.draw_calls.push(DrawCall {
            camera: camera.cloned(),
            canvas: None,
            target,
        });
        self.forward_renderer
            .require_target(&self.device, &mut self.uniforms);

        // if let Some(swapchain) = &self.swapchain {
        //     // render
        //     self.forward_renderer
        //         .render(&self.device, canvas, &cam, &mut self.uniforms, target);
        // } else {
        // }
    }

    /// Draw on a specified canvas
    ///
    /// If `camera` is `None` a default camera that fits the
    /// canvas will be used.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use duku::Duku;
    /// # let (mut duku, _) = Duku::windowed(1, 1).unwrap();
    /// let canvas = duku.create_canvas(640, 360).unwrap();
    ///
    /// duku.draw_on_canvas(&canvas, None, |t| {
    ///     // record drawing commands
    /// });
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if drawing hasn't begun.
    pub fn draw_on_canvas(
        &mut self,
        canvas: &Handle<Canvas>,
        camera: Option<&Camera>,
        draw_fn: impl Fn(&mut Target),
    ) {
        if self.render_stage == RenderStage::Before {
            panic!("cannot draw before calling 'begin'");
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins);
        draw_fn(&mut target);

        self.draw_calls.push(DrawCall {
            canvas: Some(canvas.clone()),
            camera: camera.cloned(),
            target,
        });
        self.forward_renderer
            .require_target(&self.device, &mut self.uniforms);

        // let cnv = canvas.read();
        // let cam = get_camera(camera, cnv.width, cnv.height);

        // render
        // self.forward_renderer
        // .render(&self.device, &cnv, &cam, &mut self.uniforms, target);
    }

    /// Begin this frame's drawing process
    ///
    /// # Panics
    ///
    /// Panics if drawing has already begun.
    pub fn begin(&mut self) {
        // validate render stage
        if self.render_stage == RenderStage::During {
            panic!("cannot call 'begin' 2 times in a row");
        } else {
            self.render_stage = RenderStage::During;
        }

        self.device.next_frame();
        if let Some(swapchain) = &mut self.swapchain {
            swapchain.next(&self.device);
        }
    }

    /// End this frame's drawing process
    ///
    /// # Panics
    ///
    /// Panics if drawing hasn't begun.
    pub fn end(&mut self) {
        // validate render stage
        if self.render_stage == RenderStage::Before {
            panic!("cannot call 'end' before calling 'begin'");
        } else {
            self.render_stage = RenderStage::Before;
        }

        // ready resources
        self.device.reset_buffers();
        self.resources
            .clear_unused(&self.device, &mut self.uniforms);
        self.resources
            .update_if_needed(&self.device, &mut self.uniforms);
        self.uniforms.update_if_needed(&self.device);
        self.device
            .commands()
            .bind_descriptor(&self.uniforms, self.uniforms.image_descriptor());

        // record draw calls to the command buffer
        for draw in self.draw_calls.drain(..) {
            match draw.canvas {
                Some(c) => {
                    let canvas = c.read();
                    let camera = get_camera(draw.camera.as_ref(), canvas.width, canvas.height);
                    self.forward_renderer.render(
                        &self.device,
                        &canvas,
                        &camera,
                        &mut self.uniforms,
                        draw.target,
                    );
                }
                None => {
                    let canvas = &self.window_canvases
                        [self.swapchain.as_ref().expect("bad swapchain").current()];
                    let camera = get_camera(draw.camera.as_ref(), canvas.width, canvas.height);
                    self.forward_renderer.render(
                        &self.device,
                        &canvas,
                        &camera,
                        &mut self.uniforms,
                        draw.target,
                    );
                }
            }
        }

        // submit render buffers
        self.device.submit(self.swapchain.is_some());
        self.forward_renderer.reset();

        // if has swapchain, present to it
        if let Some(swapchain) = &mut self.swapchain {
            let should_resize = self.device.present(swapchain);

            // resize if needed
            if should_resize {
                let surface = self.surface.as_ref().expect("bad surface");
                self.device.wait_idle();

                let surface_properties = self
                    .instance
                    .surface_properties(surface)
                    .remove(self.gpu_index);
                swapchain.recreate(&self.device, surface, surface_properties, self.vsync);

                for canvas in &self.window_canvases {
                    canvas.destroy(&self.device, &mut self.uniforms);
                }

                let shader_config = self.builtins.pbr_shader.read().config();
                self.window_canvases = Canvas::for_swapchain(
                    &self.device,
                    &mut self.uniforms,
                    shader_config,
                    swapchain,
                )
            }
        }

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

    /// Create a texture from byte data
    ///
    /// # Panics
    ///
    /// This function will panic if the texture limit of 100 has
    /// been reached.
    pub fn create_texture(
        &mut self,
        data: Vec<u8>,
        format: Format,
        mips: Mips,
        width: u32,
        height: u32,
    ) -> Handle<Texture> {
        let tex = Texture::new(
            &self.device,
            &mut self.uniforms,
            data,
            width,
            height,
            format,
            mips,
        );
        self.resources.add_texture(tex)
    }

    /// Create a cubemap from byte data
    ///
    /// # Panics
    ///
    /// This function will panic if the cubemap limit of 100 has
    /// been reached.
    pub fn create_cubemap(
        &mut self,
        format: Format,
        size: u32,
        sides: CubemapSides<Vec<u8>>,
    ) -> Handle<Cubemap> {
        let cub = Cubemap::new(&self.device, &mut self.uniforms, size, format, sides);
        self.resources.add_cubemap(cub)
    }

    /// Create a mesh
    pub fn create_mesh(&mut self) -> Handle<Mesh> {
        let mesh = Mesh::new(&self.device);
        self.resources.add_mesh(mesh)
    }

    /// Create a cube mesh
    pub fn create_mesh_cube(&mut self) -> Handle<Mesh> {
        self.resources
            .add_mesh(resources::create_cube(&self.device))
    }

    /// Create an ico-sphere mesh
    pub fn create_mesh_sphere_ico(&mut self, detail: u32) -> Handle<Mesh> {
        self.resources
            .add_mesh(resources::create_ico_sphere(&self.device, detail))
    }

    /// Create a uv-sphere mesh
    pub fn create_mesh_sphere_uv(&mut self, meridians: u32, parallels: u32) -> Handle<Mesh> {
        self.resources.add_mesh(resources::create_uv_sphere(
            &self.device,
            meridians,
            parallels,
        ))
    }

    /// Create a model
    pub fn create_model(&mut self) -> Handle<Model> {
        let model = Model { nodes: vec![] };
        self.resources.add_model(model)
    }

    /// Create a material
    ///
    /// # Panics
    ///
    /// This function will panic if the material limit of 100 has
    /// been reached.
    pub fn create_material(&mut self) -> Handle<Material> {
        let mat = Material::new(&self.device, &mut self.uniforms);
        self.resources.add_material(mat)
    }

    /// Create a material with PBR defaults
    ///
    /// # Panics
    ///
    /// This function will panic if the material limit of 100 has
    /// been reached.
    pub fn create_material_pbr(&mut self) -> Handle<Material> {
        let mut mat = Material::new(&self.device, &mut self.uniforms);

        mat.albedo_texture(self.builtins.white_texture.clone());
        mat.normal_texture(self.builtins.white_texture.clone());
        mat.metalness_roughness_texture(self.builtins.white_texture.clone());
        mat.ambient_occlusion_texture(self.builtins.white_texture.clone());
        mat.emissive_texture(self.builtins.black_texture.clone());
        mat.albedo_color([1.0, 1.0, 1.0]);
        mat.emissive([0.0, 0.0, 0.0]);
        mat.metalness(0.0);
        mat.roughness(0.0);
        mat.update();

        self.resources.add_material(mat)
    }

    /// Create a canvas
    ///
    /// # Panics
    ///
    /// This function will panic if the canvas limit of 100 has
    /// been reached.
    pub fn create_canvas(&mut self, width: u32, height: u32) -> Handle<Canvas> {
        let shader_config = self.builtins.pbr_shader.read().config();
        let canvas = Canvas::new(
            &self.device,
            &mut self.uniforms,
            shader_config,
            width,
            height,
        );
        self.resources.add_canvas(canvas)
    }

    /// Create a canvas with configuration based on a shader
    ///
    /// # Panics
    ///
    /// This function will panic if the canvas limit of 100 has
    /// been reached.
    pub fn create_canvas_for_shader(
        &mut self,
        shader: &Handle<Shader>,
        width: u32,
        height: u32,
    ) -> Handle<Canvas> {
        let shader_config = shader.read().config();
        let canvas = Canvas::new(
            &self.device,
            &mut self.uniforms,
            shader_config,
            width,
            height,
        );
        self.resources.add_canvas(canvas)
    }

    /// Get color data for window canvas as bytes
    /// in Rgba format
    ///
    /// # Panics
    ///
    /// This function panics if duku was created
    /// in headless mode.
    pub fn export_window_canvas(&self) -> Vec<u8> {
        self.device.wait_idle();
        self.window_canvases[self.swapchain.as_ref().expect("headless context").current()]
            .export(&self.device)
    }

    /// Get width of the window canvas
    ///
    /// # Panics
    ///
    /// This function panics if duku was created
    /// in headless mode.
    pub fn window_canvas_width(&self) -> u32 {
        self.window_canvases[self.swapchain.as_ref().expect("headless context").current()].width
    }

    /// Get height of the window canvas
    ///
    /// # Panics
    ///
    /// This function panics if duku was created
    /// in headless mode.
    pub fn window_canvas_height(&self) -> u32 {
        self.window_canvases[self.swapchain.as_ref().expect("headless context").current()].height
    }

    /// Get color data for specific canvas as bytes
    /// in Rgba format
    pub fn export_canvas(&self, canvas: &Handle<Canvas>) -> Vec<u8> {
        self.device.wait_idle();
        canvas.read().export(&self.device)
    }

    /// Create a shader from a SPIR-V file
    pub fn create_shader_spirv(&mut self, path: impl AsRef<Path>) -> Result<Handle<Shader>> {
        let bytes = fs::read(path.as_ref())?;
        self.create_shader_spirv_bytes(&bytes)
    }

    /// Create a shader from SPIR-V bytes
    pub fn create_shader_spirv_bytes(&mut self, bytes: &[u8]) -> Result<Handle<Shader>> {
        let shader = Shader::from_spirv_bytes(&self.device, &self.uniforms, self.msaa, bytes)?;
        Ok(self.resources.add_shader(shader))
    }

    /// Create a shader
    ///
    /// This should be used only if building a
    /// 3rd party shader compiler
    pub fn create_shader_bytes(
        &mut self,
        vert: &[u8],
        frag: &[u8],
        bytes: [u8; 4],
    ) -> Result<Handle<Shader>> {
        let config = ShaderConfig {
            depth: bytes[0].try_into()?,
            shape: bytes[1].try_into()?,
            cull: bytes[2].try_into()?,
            outputs: bytes[3],
            msaa: self.msaa,
        };
        let shader = Shader::new(&self.device, &self.uniforms, &vert, &frag, config)?;

        Ok(self.resources.add_shader(shader))
    }

    /// Create a font
    ///
    /// This should be used only if building a
    /// 3rd party font support
    ///
    /// # Panics
    ///
    /// This function will panic if the texture limit of 100 has
    /// been reached.
    pub fn create_font(&mut self, data: FontData<'_>) -> Handle<Font> {
        let font = Font::new(&self.device, &mut self.uniforms, data);
        self.resources.add_font(font)
    }

    /// Get last render's statistics
    pub fn stats(&self) -> Stats {
        self.device.stats()
    }

    /// Get time between frames
    pub const fn delta_time(&self) -> f32 {
        self.delta_time
    }

    /// Get current FPS
    pub const fn fps(&self) -> u32 {
        self.fps
    }
}

impl Drop for Duku {
    fn drop(&mut self) {
        self.device.wait_idle();
        self.forward_renderer
            .destroy(&self.device, &mut self.uniforms);
        for canvas in &self.window_canvases {
            canvas.destroy(&self.device, &mut self.uniforms);
        }
        self.resources.clear(&self.device, &mut self.uniforms);
        self.uniforms.destroy(&self.device);
        if let Some(swapchain) = &self.swapchain {
            self.device.destroy_swapchain(swapchain);
        }
        if let Some(surface) = &self.surface {
            self.instance.destroy_surface(surface);
        }

        self.device.destroy();
        self.instance.destroy();
    }
}

impl DukuBuilder {
    /// Use VSync setting
    pub const fn vsync(mut self, vsync: VSync) -> Self {
        self.vsync = vsync;
        self
    }

    /// Disable VSync
    pub const fn no_vsync(mut self) -> Self {
        self.vsync = VSync::Off;
        self
    }

    /// Use shadow map size
    pub const fn shadow_map_size(mut self, size: u32) -> Self {
        self.shadow_map_size = size;
        self
    }

    /// Use MSAA setting
    pub const fn msaa(mut self, msaa: Msaa) -> Self {
        self.msaa = msaa;
        self
    }

    /// Disable MSAA
    pub const fn no_msaa(mut self) -> Self {
        self.msaa = Msaa::Disabled;
        self
    }

    /// Use sampler anisotropy setting
    pub const fn anisotropy(mut self, value: f32) -> Self {
        self.anisotropy = value;
        self
    }

    /// Attach OS window handle to renderer context
    pub const fn attach_window(mut self, window: WindowHandle) -> Self {
        self.window = Some(window);
        self
    }

    /// Build context
    pub fn build(self) -> Duku {
        let Self {
            vsync,
            msaa,
            anisotropy,
            shadow_map_size,
            window,
        } = self;

        let instance = Instance::new();
        let surface = window.map(|w| Surface::new(&instance, w));

        // query properties
        let mut gpu_properties_list = instance.gpu_properties();
        let surface_properties_list = surface.as_ref().map(|s| instance.surface_properties(&s));

        // choose GPU
        let gpu_index = pick_gpu(&gpu_properties_list, &surface_properties_list, vsync, msaa);

        let gpu_properties = gpu_properties_list.remove(gpu_index);
        let device = Device::new(&instance, gpu_properties, gpu_index);

        let swapchain = surface.as_ref().map(|s| {
            let surface_properties = surface_properties_list
                .expect("bad properties")
                .remove(gpu_index);
            Swapchain::new(&device, &s, surface_properties, vsync)
        });

        info!("using anisotropy level {}", anisotropy);
        info!("using msaa level {:?}", msaa);
        info!("using vsync {:?}", vsync);

        // setup uniforms
        let mut uniforms = Uniforms::new(&device, anisotropy);

        // setup resources
        let mut resources = Resources::default();
        let builtins = Builtins::new(&device, &mut resources, &mut uniforms, msaa);

        // setup canvases
        let shader_config = builtins.pbr_shader.read().config();
        let window_canvases = swapchain
            .as_ref()
            .map(|s| Canvas::for_swapchain(&device, &mut uniforms, shader_config, &s))
            .unwrap_or_default();

        // setup renderer
        let forward_renderer = ForwardRenderer::new(&device, &mut uniforms, shadow_map_size);

        Duku {
            fps_samples: [0; FPS_SAMPLE_COUNT],
            render_stage: RenderStage::Before,
            frame_time: Instant::now(),
            fps: 0,
            delta_time: 0.0,
            frame_count: 0,
            draw_calls: vec![],
            window_canvases,
            forward_renderer,
            builtins,
            uniforms,
            resources,
            swapchain,
            gpu_index,
            instance,
            surface,
            device,
            msaa,
            vsync,
        }
    }
}

fn get_camera(camera: Option<&Camera>, width: u32, height: u32) -> Camera {
    match camera {
        Some(c) => {
            if c.width.is_none() || c.height.is_none() {
                let mut cam =
                    Camera::new(c.projection, width as f32, height as f32, c.depth, c.fov);
                cam.position = c.position;
                cam.scale = c.scale;
                cam.rotation = c.rotation;
                cam
            } else {
                c.clone()
            }
        }
        // create default camera if not supplied
        None => Camera::new(
            Projection::Orthographic,
            width as f32,
            height as f32,
            100.0,
            90,
        ),
    }
}
