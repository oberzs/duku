use ash::vk::DescriptorSet;
use ash::vk::Pipeline;
use std::rc::Rc;
use std::rc::Weak;
use tegne_math::Camera;
use tegne_math::Matrix4;
use tegne_math::Vector3;

use crate::builtins::BuiltinMaterial;
use crate::builtins::Builtins;
use crate::images::Framebuffer;
use crate::instance::Device;
use crate::instance::Order;
use crate::instance::Target;
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
    pub(crate) builtins: &'a Builtins,
    pub(crate) target: Target<'a>,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Rc<Device>,
        depth_pass: &RenderPass,
        image_uniforms: &ImageUniforms,
        shader_layout: &ShaderLayout,
    ) -> Self {
        let shadow_framebuffer = Framebuffer::depth(
            device,
            depth_pass,
            image_uniforms,
            shader_layout,
            2048,
            2048,
        );

        Self {
            shadow_framebuffer,
            device: Rc::downgrade(device),
        }
    }

    pub fn draw(&self, options: ForwardDrawOptions) {
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
            time: 0.0,
        };

        let clear = options.target.clear();

        let device = self.device();
        let recorder = device.record_commands();

        // shadow mapping
        recorder.begin_render_pass(&self.shadow_framebuffer, options.depth_pass, clear);
        self.setup_pass(&self.shadow_framebuffer);
        self.bind_world(&self.shadow_framebuffer, world_object, &options);

        let shadow_material = options.builtins.get_material(BuiltinMaterial::Shadow);
        self.bind_material(
            shadow_material.pipeline(),
            shadow_material.uniforms().descriptor(),
            &options,
        );

        for mat_order in options.target.material_orders() {
            for order in mat_order.orders.iter() {
                if order.has_shadows {
                    self.draw_order_with_albedo(order, &options, shadow_material.albedo_index());
                }
            }
        }

        recorder.end_render_pass();
        self.shadow_framebuffer.blit_to_shader_image(&recorder);

        // normal render
        recorder.begin_render_pass(options.framebuffer, options.color_pass, clear);
        self.setup_pass(options.framebuffer);
        self.bind_world(options.framebuffer, world_object, &options);

        for mat_order in options.target.material_orders() {
            self.bind_material(mat_order.pipeline, mat_order.material_descriptor, &options);

            for order in mat_order.orders.iter() {
                self.draw_order(order, &options);
            }
        }

        recorder.end_render_pass();
    }

    fn setup_pass(&self, framebuffer: &Framebuffer) {
        let device = self.device();
        let recorder = device.record_commands();

        recorder.set_view(framebuffer.width(), framebuffer.height());
        recorder.set_line_width(1.0);
    }

    fn bind_world(
        &self,
        framebuffer: &Framebuffer,
        object: WorldObject,
        options: &ForwardDrawOptions,
    ) {
        let device = self.device();
        let recorder = device.record_commands();

        framebuffer.world_uniforms().update(object);
        recorder.bind_descriptor(
            framebuffer.world_uniforms().descriptor(),
            options.shader_layout.pipeline(),
        );
    }

    fn bind_material(
        &self,
        pipeline: Pipeline,
        descriptor: (u32, DescriptorSet),
        options: &ForwardDrawOptions,
    ) {
        let device = self.device();
        let recorder = device.record_commands();

        recorder.bind_pipeline(pipeline);
        recorder.bind_descriptor(descriptor, options.shader_layout.pipeline());
    }

    fn draw_order(&self, order: &Order, options: &ForwardDrawOptions) {
        self.draw_order_with_albedo(order, options, order.albedo_index);
    }

    fn draw_order_with_albedo(
        &self,
        order: &Order,
        options: &ForwardDrawOptions,
        albedo_index: i32,
    ) {
        let device = self.device();
        let recorder = device.record_commands();

        recorder.set_push_constant(
            PushConstants {
                model_mat: order.model,
                albedo_index,
            },
            options.shader_layout.pipeline(),
        );
        recorder.bind_vertex_buffer(order.vertex_buffer);
        recorder.bind_index_buffer(order.index_buffer);
        recorder.draw(order.index_count);
    }

    fn device(&self) -> Rc<Device> {
        self.device.upgrade().expect("device does not exist")
    }
}
