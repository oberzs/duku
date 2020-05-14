use ash::vk::Pipeline;
use std::cell::Ref;
use std::sync::Arc;
use std::sync::Weak;
use tegne_math::Camera;
use tegne_math::Matrix4;
use tegne_math::Vector3;

use crate::error::ErrorKind;
use crate::error::Result;
use crate::images::Framebuffer;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::Order;
use crate::instance::Target;
use crate::objects::BuiltinShader;
use crate::objects::Objects;
use crate::shaders::Descriptor;
use crate::shaders::ImageUniforms;
use crate::shaders::PushConstants;
use crate::shaders::RenderPass;
use crate::shaders::ShaderLayout;
use crate::shaders::WorldObject;

pub(crate) struct ForwardRenderer {
    shadow_framebuffer: Framebuffer,
    device: Weak<Device>,
}

pub(crate) struct ForwardDrawOptions<'a> {
    pub(crate) framebuffer: &'a Framebuffer,
    pub(crate) depth_pass: &'a RenderPass,
    pub(crate) color_pass: &'a RenderPass,
    pub(crate) shader_layout: &'a ShaderLayout,
    pub(crate) camera: &'a Camera,
    pub(crate) objects: &'a Objects,
    pub(crate) target: Target<'a>,
    pub(crate) time: f32,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        depth_pass: &RenderPass,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
    ) -> Result<Self> {
        let shadow_framebuffer = Framebuffer::depth(
            device,
            depth_pass,
            image_uniforms,
            shader_layout,
            2048,
            2048,
        )?;

        Ok(Self {
            shadow_framebuffer,
            device: Arc::downgrade(device),
        })
    }

    pub fn draw(&self, options: ForwardDrawOptions<'_>) -> Result<()> {
        let cam_mat = options.camera.matrix();

        let light_distance = 10.0;
        let light_dir = options.target.lights()[0].coords.shrink();
        let light_mat_dir = light_dir.unit();
        let light_mat_pos = light_mat_dir * light_distance;
        let light_mat = Matrix4::orthographic(20.0, 20.0, 0.1, 50.0)
            * Matrix4::look_rotation(light_mat_dir, Vector3::up())
            * Matrix4::translation(light_mat_pos);

        let world_object = WorldObject {
            cam_mat,
            cam_pos: options.camera.transform().position,
            lights: options.target.lights(),
            light_mat,
            shadow_index: self.shadow_framebuffer.image_index(),
            time: options.time,
        };

        let clear = options.target.clear();

        let device = self.device.upgrade().ok_or(ErrorKind::DeviceDropped)?;
        let cmd = device.commands();

        // shadow mapping
        cmd.begin_render_pass(&self.shadow_framebuffer, options.depth_pass, clear)?;
        self.setup_pass(&cmd, &self.shadow_framebuffer)?;
        self.bind_world(&cmd, &self.shadow_framebuffer, world_object, &options)?;

        let shadow_shader = options.objects.builtins().shader(BuiltinShader::Shadow);
        self.bind_shader(&cmd, shadow_shader.pipeline())?;

        for s_order in options.target.orders_by_shader() {
            for m_order in s_order.orders_by_material() {
                self.bind_material(&cmd, m_order.material(), &options)?;
                for order in m_order.orders() {
                    if order.has_shadows {
                        self.draw_order(&cmd, order, &options)?;
                    }
                }
            }
        }

        cmd.end_render_pass()?;
        self.shadow_framebuffer.blit_to_shader_image(&cmd)?;

        // normal render
        cmd.begin_render_pass(options.framebuffer, options.color_pass, clear)?;
        self.setup_pass(&cmd, options.framebuffer)?;
        self.bind_world(&cmd, options.framebuffer, world_object, &options)?;

        for s_order in options.target.orders_by_shader() {
            self.bind_shader(&cmd, s_order.shader())?;
            for m_order in s_order.orders_by_material() {
                self.bind_material(&cmd, m_order.material(), &options)?;
                for order in m_order.orders() {
                    self.draw_order(&cmd, order, &options)?;
                }
            }
        }

        // wireframe render
        let wireframe_shader = options.objects.builtins().shader(BuiltinShader::Wireframe);
        self.bind_shader(&cmd, wireframe_shader.pipeline())?;
        for order in options.target.wireframe_orders() {
            self.draw_order(&cmd, order, &options)?;
        }

        cmd.end_render_pass()?;

        Ok(())
    }

    fn setup_pass(&self, cmd: &Ref<'_, Commands>, framebuffer: &Framebuffer) -> Result<()> {
        cmd.set_view(framebuffer.width(), framebuffer.height())?;
        cmd.set_line_width(1.0)?;
        Ok(())
    }

    fn bind_world(
        &self,
        cmd: &Ref<'_, Commands>,
        framebuffer: &Framebuffer,
        object: WorldObject,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        framebuffer.world_uniforms().update(object)?;
        cmd.bind_descriptor(
            framebuffer.world_uniforms().descriptor(),
            options.shader_layout.pipeline(),
        )?;
        Ok(())
    }

    fn bind_shader(&self, cmd: &Ref<'_, Commands>, shader: Pipeline) -> Result<()> {
        cmd.bind_pipeline(shader)?;
        Ok(())
    }

    fn bind_material(
        &self,
        cmd: &Ref<'_, Commands>,
        descriptor: Descriptor,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        cmd.bind_descriptor(descriptor, options.shader_layout.pipeline())?;
        Ok(())
    }

    fn draw_order(
        &self,
        cmd: &Ref<'_, Commands>,
        order: Order,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        cmd.set_push_constant(
            PushConstants {
                model_mat: order.model,
                albedo_index: order.albedo_index,
            },
            options.shader_layout.pipeline(),
        )?;
        cmd.bind_vertex_buffer(order.vertex_buffer)?;
        cmd.bind_index_buffer(order.index_buffer)?;
        cmd.draw(order.index_count)?;
        Ok(())
    }
}
