use std::sync::Arc;
use tegne_math::Camera;
use tegne_math::Matrix4;
use tegne_math::Vector3;

use crate::error::Result;
use crate::images::Framebuffer;
use crate::instance::Commands;
use crate::instance::Device;
use crate::instance::Order;
use crate::instance::Target;
use crate::objects::Builtins;
use crate::objects::IdRef;
use crate::objects::Objects;
use crate::shaders::ImageUniforms;
use crate::shaders::PushConstants;
use crate::shaders::RenderPass;
use crate::shaders::RenderPasses;
use crate::shaders::ShaderLayout;
use crate::shaders::WorldObject;

pub(crate) struct ForwardRenderer {
    shadow_framebuffer: Framebuffer,
}

pub(crate) struct ForwardDrawOptions<'a> {
    pub(crate) framebuffer: &'a Framebuffer,
    pub(crate) render_passes: &'a RenderPasses,
    pub(crate) color_pass: &'a RenderPass,
    pub(crate) shader_layout: &'a ShaderLayout,
    pub(crate) camera: &'a Camera,
    pub(crate) objects: &'a Objects,
    pub(crate) builtins: &'a Builtins,
    pub(crate) cmd: &'a Commands,
    pub(crate) target: Target<'a>,
    pub(crate) time: f32,
    pub(crate) blit: bool,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        render_passes: &RenderPasses,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
    ) -> Result<Self> {
        let shadow_framebuffer = Framebuffer::depth(
            device,
            render_passes,
            image_uniforms,
            shader_layout,
            2048,
            2048,
        )?;

        Ok(Self { shadow_framebuffer })
    }

    pub fn draw(&self, options: ForwardDrawOptions<'_>) -> Result<()> {
        let depth_pass = options.render_passes.depth();

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

        let cmd = options.cmd;

        // shadow mapping
        cmd.begin_render_pass(&self.shadow_framebuffer, depth_pass, clear);
        self.setup_pass(&self.shadow_framebuffer, &options);
        self.bind_world(&self.shadow_framebuffer, world_object, &options)?;

        self.bind_shader(options.builtins.shaders.shadow.id_ref(), &options);
        for s_order in options.target.orders_by_shader() {
            for m_order in s_order.orders_by_material() {
                self.bind_material(m_order.material(), &options)?;
                for order in m_order.orders() {
                    if order.has_shadows {
                        self.draw_order(order, &options)?;
                    }
                }
            }
        }

        cmd.end_render_pass();
        self.shadow_framebuffer.blit_to_shader_image(cmd);

        // normal render
        cmd.begin_render_pass(options.framebuffer, options.color_pass, clear);
        self.setup_pass(options.framebuffer, &options);
        self.bind_world(options.framebuffer, world_object, &options)?;

        for s_order in options.target.orders_by_shader() {
            self.bind_shader(s_order.shader(), &options);
            for m_order in s_order.orders_by_material() {
                self.bind_material(m_order.material(), &options)?;
                for order in m_order.orders() {
                    self.draw_order(order, &options)?;
                }
            }
        }

        // wireframe render
        self.bind_shader(options.builtins.shaders.wireframe.id_ref(), &options);
        for order in options.target.wireframe_orders() {
            self.draw_order(order, &options)?;
        }

        cmd.end_render_pass();
        if options.blit {
            options.framebuffer.blit_to_shader_image(cmd);
        }

        Ok(())
    }

    fn setup_pass(&self, framebuffer: &Framebuffer, options: &ForwardDrawOptions<'_>) {
        let cmd = options.cmd;
        cmd.set_view(framebuffer.width(), framebuffer.height());
        cmd.set_line_width(1.0);
    }

    fn bind_world(
        &self,
        framebuffer: &Framebuffer,
        object: WorldObject,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        let cmd = options.cmd;
        framebuffer.world_uniforms().update(object)?;
        cmd.bind_descriptor(
            framebuffer.world_uniforms().descriptor(),
            options.shader_layout.pipeline(),
        );
        Ok(())
    }

    fn bind_shader(&self, shader: IdRef, options: &ForwardDrawOptions<'_>) {
        let cmd = options.cmd;
        let objects = options.objects;
        if let Some(pipeline) = objects.with_shader(shader, |s| s.pipeline()) {
            cmd.bind_pipeline(pipeline);
        }
    }

    fn bind_material(&self, material: IdRef, options: &ForwardDrawOptions<'_>) -> Result<()> {
        let cmd = options.cmd;
        let objects = options.objects;
        if let Some(descriptor) = objects.with_material(material, |m| m.descriptor()) {
            cmd.bind_descriptor(descriptor?, options.shader_layout.pipeline());
        }
        Ok(())
    }

    fn draw_order(&self, order: Order, options: &ForwardDrawOptions<'_>) -> Result<()> {
        let cmd = options.cmd;
        let objects = options.objects;
        let albedo = objects
            .with_texture(order.albedo, |t| t.image_index())
            .or_else(|| objects.with_framebuffer(order.albedo, |f| f.image_index()));
        if let Some(albedo_index) = albedo {
            if let Some((vb, ib, n)) = objects.with_mesh(order.mesh, |m| {
                (
                    m.vk_vertex_buffer(),
                    m.vk_index_buffer(),
                    m.drawn_triangles() * 3,
                )
            }) {
                cmd.set_push_constant(
                    PushConstants {
                        model_mat: order.model,
                        albedo_index,
                    },
                    options.shader_layout.pipeline(),
                );
                cmd.bind_vertex_buffer(vb?);
                cmd.bind_index_buffer(ib);
                cmd.draw(n);
            }
        }
        Ok(())
    }
}
