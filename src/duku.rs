// Oliver Berzs
// https://github.com/oberzs/duku

use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::device::pick_gpu;
use crate::device::Device;
use crate::device::Stats;
use crate::error::Result;
use crate::font::Font;
use crate::image::Cubemap;
use crate::image::CubemapSides;
use crate::image::Format;
use crate::image::Framebuffer;
use crate::image::Mips;
use crate::image::Msaa;
use crate::image::Size;
use crate::image::Texture;
use crate::instance::Instance;
use crate::mesh::Mesh;
use crate::mesh::MeshBuilder;
use crate::mesh::Model;
use crate::mesh::ModelNode;
use crate::pipeline::Material;
use crate::pipeline::MaterialBuilder;
use crate::pipeline::Shader;
use crate::pipeline::Uniforms;
use crate::renderer::Camera;
use crate::renderer::Color;
use crate::renderer::ForwardRenderer;
use crate::renderer::Target;
use crate::storage;
use crate::storage::Builtins;
use crate::storage::Handle;
use crate::storage::Storage;
use crate::surface::Surface;
use crate::surface::Swapchain;
use crate::surface::VSync;
use crate::surface::WindowHandle;

const FPS_SAMPLE_COUNT: usize = 64;

pub struct Duku {
    // Vulkan
    instance: Instance,
    device: Device,
    gpu_index: usize,
    surface: Surface,
    swapchain: Swapchain,
    uniforms: Uniforms,
    window_framebuffers: Vec<Framebuffer>,

    // Resources
    storage: Storage,
    pub builtins: Builtins,

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
}

#[derive(Debug, Clone)]
pub struct DukuBuilder {
    shadow_map_size: u32,
    anisotropy: f32,
    msaa: Msaa,
    vsync: VSync,
    window: Option<WindowHandle>,
}

#[derive(Copy, Clone)]
enum RenderStage {
    Before,
    During,
}

impl Duku {
    pub const fn builder() -> DukuBuilder {
        DukuBuilder {
            shadow_map_size: 2048,
            anisotropy: 4.0,
            msaa: Msaa::X4,
            vsync: VSync::On,
            window: None,
        }
    }

    pub fn draw_on_window<F>(&mut self, camera: Option<&Camera>, draw_callback: F)
    where
        F: Fn(&mut Target<'_>),
    {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        // let user record draw calls
        {
            let mut target = Target::new(&self.builtins, &self.storage);
            draw_callback(&mut target);
            let framebuffer = &self.window_framebuffers[self.swapchain.current()];
            let cam = get_camera(camera, framebuffer.size());
            // render
            self.forward_renderer
                .render(&self.device, framebuffer, &cam, &self.uniforms, target);
        }

        self.end_draw();
    }

    pub fn draw<F>(
        &mut self,
        framebuffer: &Handle<Framebuffer>,
        camera: Option<&Camera>,
        draw_callback: F,
    ) where
        F: Fn(&mut Target<'_>),
    {
        if let RenderStage::Before = self.render_stage {
            self.begin_draw();
        }

        // let user record draw calls
        let mut target = Target::new(&self.builtins, &self.storage);
        draw_callback(&mut target);

        let frame = self.storage.framebuffers.get(framebuffer);
        let cam = get_camera(camera, frame.size());

        // render
        self.forward_renderer
            .render(&self.device, frame, &cam, &self.uniforms, target);
    }

    pub fn create_texture(
        &mut self,
        data: Vec<u8>,
        format: Format,
        mips: Mips,
        width: u32,
        height: u32,
    ) -> Result<Handle<Texture>> {
        let tex = Texture::new(
            &self.device,
            &mut self.uniforms,
            data,
            Size::new(width, height),
            format,
            mips,
        )?;
        Ok(self.storage.add_texture(tex))
    }

    pub fn create_texture_color(
        &mut self,
        pixels: &[Color],
        width: u32,
        height: u32,
    ) -> Result<Handle<Texture>> {
        let data: Vec<_> = pixels
            .iter()
            .map(|p| vec![p.r, p.g, p.b, p.a])
            .flatten()
            .collect();

        self.create_texture(data, Format::Rgba, Mips::Zero, width, height)
    }

    pub fn texture(&self, tex: &Handle<Texture>) -> &Texture {
        self.storage.textures.get(tex)
    }

    pub fn texture_mut(&mut self, tex: &Handle<Texture>) -> &mut Texture {
        self.storage.textures.get_mut(tex)
    }

    pub fn create_cubemap(
        &mut self,
        format: Format,
        size: u32,
        sides: CubemapSides<Vec<u8>>,
    ) -> Result<Handle<Cubemap>> {
        let cub = Cubemap::new(&self.device, &mut self.uniforms, size, format, sides)?;
        Ok(self.storage.add_cubemap(cub))
    }

    pub fn font(&self, font: &Handle<Font>) -> &Font {
        self.storage.fonts.get(font)
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
        result.set_vertices(m.vertices().copied().collect());
        result.set_normals(m.normals().copied().collect());
        result.set_colors(m.colors().copied().collect());
        result.set_uvs(m.uvs().copied().collect());
        result.set_indices(m.indices().copied().collect());
        self.storage.add_mesh(result)
    }

    pub fn combine_meshes(&mut self, meshes: &[Handle<Mesh>]) -> Handle<Mesh> {
        let ms: Vec<_> = meshes.iter().map(|m| self.storage.meshes.get(m)).collect();
        let mesh = Mesh::combine(&self.device, &ms);
        self.storage.add_mesh(mesh)
    }

    pub fn create_model(&mut self, nodes: Vec<ModelNode>) -> Handle<Model> {
        let model = Model { nodes };
        self.storage.add_model(model)
    }

    pub fn model(&self, model: &Handle<Model>) -> &Model {
        self.storage.models.get(model)
    }

    pub fn model_mut(&mut self, model: &Handle<Model>) -> &mut Model {
        self.storage.models.get_mut(model)
    }

    pub fn fix_model_color_space(&mut self, model: &Handle<Model>) {
        let mdl = self.storage.models.get(model);
        for material in mdl.materials() {
            let mat = self.storage.materials.get_mut(material);
            mat.fix_albedo_color_space();
        }
    }

    pub fn create_material(&mut self) -> Result<Handle<Material>> {
        let mat = Material::new(&self.device, &mut self.uniforms)?;
        Ok(self.storage.add_material(mat))
    }

    pub fn material(&self, material: &Handle<Material>) -> &Material {
        self.storage.materials.get(material)
    }

    pub fn material_mut(&mut self, material: &Handle<Material>) -> &mut Material {
        self.storage.materials.get_mut(material)
    }

    pub fn build_material(&mut self) -> Result<MaterialBuilder<'_>> {
        Ok(MaterialBuilder {
            storage: &mut self.storage,
            material: Material::new(&self.device, &mut self.uniforms)?,
        }
        .albedo_texture(self.builtins.white_texture.clone())
        .normal_texture(self.builtins.blue_texture.clone())
        .metalness_roughness_texture(self.builtins.white_texture.clone())
        .ambient_occlusion_texture(self.builtins.white_texture.clone())
        .emissive_texture(self.builtins.black_texture.clone())
        .albedo_color([255, 255, 255])
        .emissive([0, 0, 0])
        .metalness(0.0)
        .roughness(0.0))
    }

    pub fn create_framebuffer(&mut self, width: u32, height: u32) -> Result<Handle<Framebuffer>> {
        let shader_config = self.storage.shaders.get(&self.builtins.pbr_shader).config();
        let framebuffer = Framebuffer::new(
            &self.device,
            &mut self.uniforms,
            shader_config,
            Size::new(width, height),
        )?;
        self.forward_renderer
            .add_target(&self.device, &mut self.uniforms)?;
        Ok(self.storage.add_framebuffer(framebuffer))
    }

    pub fn create_framebuffer_for_shader(
        &mut self,
        shader: &Handle<Shader>,
        width: u32,
        height: u32,
    ) -> Result<Handle<Framebuffer>> {
        let shader_config = self.storage.shaders.get(shader).config();
        let framebuffer = Framebuffer::new(
            &self.device,
            &mut self.uniforms,
            shader_config,
            Size::new(width, height),
        )?;
        self.forward_renderer
            .add_target(&self.device, &mut self.uniforms)?;
        Ok(self.storage.add_framebuffer(framebuffer))
    }

    pub fn framebuffer_mut(&mut self, framebuffer: &Handle<Framebuffer>) -> &mut Framebuffer {
        self.storage.framebuffers.get_mut(framebuffer)
    }

    pub fn create_shader_spirv(&mut self, path: impl AsRef<Path>) -> Result<Handle<Shader>> {
        let bytes = fs::read(path.as_ref())?;
        self.create_shader_spirv_bytes(&bytes)
    }

    pub fn create_shader_spirv_bytes(&mut self, bytes: &[u8]) -> Result<Handle<Shader>> {
        let shader = Shader::from_spirv_bytes(&self.device, &self.uniforms, self.msaa, bytes)?;
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

    pub(crate) const fn msaa(&self) -> Msaa {
        self.msaa
    }

    pub(crate) const fn device(&self) -> &Device {
        &self.device
    }

    pub(crate) const fn uniforms(&self) -> &Uniforms {
        &self.uniforms
    }

    pub(crate) fn storage_mut(&mut self) -> &mut Storage {
        &mut self.storage
    }

    fn begin_draw(&mut self) {
        self.render_stage = RenderStage::During;
        self.device.next_frame(&mut self.swapchain);
        self.storage.clear_unused(&self.device, &mut self.uniforms);
        self.storage
            .update_if_needed(&self.device, &mut self.uniforms);
        self.uniforms.update_if_needed(&self.device);
        self.device
            .commands()
            .bind_descriptor(&self.uniforms, self.uniforms.image_descriptor());
    }

    fn end_draw(&mut self) {
        self.render_stage = RenderStage::Before;
        self.device.submit();
        let should_resize = self.device.present(&self.swapchain);

        // update delta time
        let delta_time = self.frame_time.elapsed();
        self.delta_time = delta_time.as_secs_f32();
        self.frame_time = Instant::now();
        self.fps_samples[self.frame_count % FPS_SAMPLE_COUNT] =
            1_000_000 / delta_time.as_micros() as u32;
        self.frame_count += 1;
        self.fps =
            (self.fps_samples.iter().sum::<u32>() as f32 / FPS_SAMPLE_COUNT as f32).ceil() as u32;

        // resize if needed
        if should_resize {
            self.device.wait_idle();

            let gpu_properties = self
                .instance
                .gpu_properties(&self.surface)
                .remove(self.gpu_index);
            self.swapchain
                .recreate(&self.device, &self.surface, &gpu_properties, self.vsync);

            for framebuffer in &self.window_framebuffers {
                framebuffer.destroy(&self.device, &mut self.uniforms);
            }

            let shader_config = self.storage.shaders.get(&self.builtins.pbr_shader).config();
            self.window_framebuffers =
                Framebuffer::for_swapchain(&self.device, shader_config, &self.swapchain);
        }
    }
}

impl Drop for Duku {
    fn drop(&mut self) {
        self.device.wait_idle();
        self.storage.clear(&self.device, &mut self.uniforms);
        self.forward_renderer
            .destroy(&self.device, &mut self.uniforms);
        for framebuffer in &self.window_framebuffers {
            framebuffer.destroy(&self.device, &mut self.uniforms);
        }
        self.uniforms.destroy(&self.device);
        self.device.destroy_swapchain(&self.swapchain);
        self.instance.destroy_surface(&self.surface);

        self.device.destroy();
        self.instance.destroy();
    }
}

impl DukuBuilder {
    pub const fn vsync(mut self, vsync: VSync) -> Self {
        self.vsync = vsync;
        self
    }

    pub const fn no_vsync(mut self) -> Self {
        self.vsync = VSync::Off;
        self
    }

    pub const fn shadow_map_size(mut self, size: u32) -> Self {
        self.shadow_map_size = size;
        self
    }

    pub const fn msaa(mut self, msaa: Msaa) -> Self {
        self.msaa = msaa;
        self
    }

    pub const fn no_msaa(mut self) -> Self {
        self.msaa = Msaa::Disabled;
        self
    }

    pub const fn anisotropy(mut self, value: f32) -> Self {
        self.anisotropy = value;
        self
    }

    pub const fn attach_window(mut self, window: WindowHandle) -> Self {
        self.window = Some(window);
        self
    }

    pub fn build(self) -> Result<Duku> {
        let Self {
            vsync,
            msaa,
            anisotropy,
            shadow_map_size,
            window,
        } = self;

        let window_handle = match window {
            Some(w) => w,
            None => unimplemented!(),
        };
        let instance = Instance::new();
        let surface = Surface::new(&instance, window_handle);

        // setup device stuff
        let mut gpu_properties_list = instance.gpu_properties(&surface);
        let gpu_index = pick_gpu(&gpu_properties_list, vsync, msaa)?;
        let gpu_properties = gpu_properties_list.remove(gpu_index);
        let device = Device::new(&instance, &gpu_properties, gpu_index);
        let swapchain = Swapchain::new(&device, &surface, &gpu_properties, vsync);

        info!("using anisotropy level {}", anisotropy);
        info!("using msaa level {:?}", msaa);
        info!("using vsync {:?}", vsync);

        // setup uniforms
        let mut uniforms = Uniforms::new(&device, anisotropy);

        // setup storage
        let mut storage = Storage::new();
        let builtins = Builtins::new(&device, &mut storage, &mut uniforms, msaa)?;

        // setup framebuffers
        let shader_config = storage.shaders.get(&builtins.pbr_shader).config();
        let window_framebuffers = Framebuffer::for_swapchain(&device, shader_config, &swapchain);

        // setup renderer
        let forward_renderer = ForwardRenderer::new(
            &device,
            &mut uniforms,
            shadow_map_size,
            gpu_properties.image_count,
        )?;

        Ok(Duku {
            fps_samples: [0; FPS_SAMPLE_COUNT],
            render_stage: RenderStage::Before,
            frame_time: Instant::now(),
            fps: 0,
            delta_time: 0.0,
            frame_count: 0,
            window_framebuffers,
            forward_renderer,
            uniforms,
            storage,
            swapchain,
            gpu_index,
            builtins,
            instance,
            surface,
            device,
            msaa,
            vsync,
        })
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
