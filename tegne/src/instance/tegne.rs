use log::debug;
use std::collections::HashMap;
use std::rc::Rc;
use tegne_math::Camera;
use tegne_math::Matrix4;
use tegne_math::Vector3;

use super::Device;
use super::Extensions;
use super::Swapchain;
use super::Target;
use super::Validator;
use super::Vulkan;
use super::WindowArgs;
use super::WindowSurface;
use crate::builtins::BuiltinMaterial;
use crate::builtins::BuiltinShader;
use crate::builtins::BuiltinTexture;
use crate::builtins::Builtins;
use crate::images::Framebuffer;
use crate::images::Texture;
use crate::mesh::Mesh;
use crate::mesh::MeshBuilder;
use crate::shaders::ImageUniforms;
use crate::shaders::Material;
use crate::shaders::MaterialBuilder;
use crate::shaders::PushConstants;
use crate::shaders::RenderPass;
use crate::shaders::Shader;
use crate::shaders::ShaderBuilder;
use crate::shaders::ShaderLayout;
use crate::shaders::WorldObject;
use crate::utils::OrError;

#[cfg(feature = "tegne-utils")]
use tegne_utils::Window;

pub struct Tegne {
    builtins: Builtins,
    window_framebuffers: Vec<Framebuffer>,
    shadow_framebuffer: Framebuffer,
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

    pub fn draw_on_window(&self, camera: &Camera, draw_callback: impl Fn(&mut Target)) {
        let mut target = Target::new(&self.builtins);
        draw_callback(&mut target);

        let (proj, view) = camera.matrices();
        let light_pos = Vector3::new(5.0, 5.0, -5.0);
        let light_matrix = Matrix4::orthographic(10.0, 10.0, 0.1, 100.0)
            * Matrix4::look_rotation(-light_pos, Vector3::up())
            * Matrix4::translation(-light_pos);

        let clear = target.clear();

        let framebuffer = &self.window_framebuffers[self.swapchain.current()];
        let shadow_framebuffer = &self.shadow_framebuffer;
        let window_pass = self
            .render_passes
            .get(&RenderPassType::Window)
            .or_error("render passes not setup");
        let depth_pass = self
            .render_passes
            .get(&RenderPassType::Depth)
            .or_error("render passes not setup");

        let world_uniforms = framebuffer.world_uniforms();
        world_uniforms.update(WorldObject {
            proj,
            view,
            lights: target.lights(),
            light_matrix,
            view_pos: camera.transform().position,
            time: 0.0,
        });

        let recorder = self.device.record_commands();

        recorder.bind_descriptor(world_uniforms.descriptor(), self.shader_layout.pipeline());

        // shadow mapping
        recorder.begin_render_pass(shadow_framebuffer, depth_pass, clear);
        recorder.set_view(shadow_framebuffer.width(), shadow_framebuffer.height());
        recorder.set_line_width(1.0);

        let shadow_material = self.builtins.get_material(BuiltinMaterial::Shadow);
        recorder.bind_pipeline(shadow_material.pipeline());
        recorder.bind_descriptor(
            shadow_material.uniforms().descriptor(),
            self.shader_layout.pipeline(),
        );

        for mat_order in target.material_orders() {
            for order in mat_order.orders.iter() {
                recorder.set_push_constant(
                    PushConstants {
                        model: order.model,
                        albedo_index: shadow_material.albedo_index(),
                    },
                    self.shader_layout.pipeline(),
                );

                recorder.bind_vertex_buffer(order.vertex_buffer);
                recorder.bind_index_buffer(order.index_buffer);

                recorder.draw(order.index_count);
            }
        }

        recorder.end_render_pass();
        shadow_framebuffer.blit_to_shader_image(&recorder);

        // normal render
        recorder.begin_render_pass(framebuffer, window_pass, clear);
        recorder.set_view(framebuffer.width(), framebuffer.height());
        recorder.set_line_width(1.0);

        for mat_order in target.material_orders() {
            recorder.bind_pipeline(mat_order.pipeline);
            recorder.bind_descriptor(mat_order.material_descriptor, self.shader_layout.pipeline());

            for order in mat_order.orders.iter() {
                recorder.set_push_constant(
                    PushConstants {
                        model: order.model,
                        albedo_index: mat_order.albedo_index,
                    },
                    self.shader_layout.pipeline(),
                );

                recorder.bind_vertex_buffer(order.vertex_buffer);
                recorder.bind_index_buffer(order.index_buffer);

                recorder.draw(order.index_count);
            }
        }

        recorder.end_render_pass();
    }

    pub fn create_texture_rgba(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("creating rgba texture");
        let texture =
            Texture::from_raw_rgba(&self.device, raw, width, height, &self.image_uniforms);
        texture
    }

    pub fn create_texture_rgb(&self, raw: &[u8], width: u32, height: u32) -> Texture {
        debug!("creating rgb texture");
        let texture = Texture::from_raw_rgb(&self.device, raw, width, height, &self.image_uniforms);
        texture
    }

    pub fn create_mesh(&self) -> MeshBuilder {
        debug!("creating mesh");
        Mesh::builder(&self.device)
    }

    pub fn create_material(&self) -> MaterialBuilder {
        debug!("creating material");
        let default_shader = self.builtins.get_shader(BuiltinShader::Phong);
        let default_texture = self.builtins.get_texture(BuiltinTexture::White);
        Material::builder(
            &self.device,
            default_shader,
            default_texture,
            &self.shader_layout,
        )
    }

    pub fn create_framebuffer(&self, width: u32, height: u32) -> Framebuffer {
        debug!("creating framebuffer");
        let render_pass = self
            .render_passes
            .get(&RenderPassType::Color)
            .or_error("render passes not setup");
        Framebuffer::new(
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

        let window_framebuffers = Framebuffer::for_window(
            &device,
            &swapchain,
            &window_pass,
            &image_uniforms,
            &shader_layout,
            window_args.width,
            window_args.height,
        );
        let shadow_framebuffer = Framebuffer::new(
            &device,
            &depth_pass,
            &image_uniforms,
            &shader_layout,
            window_args.width,
            window_args.height,
        );

        Tegne {
            builtins,
            window_framebuffers,
            shadow_framebuffer,
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
