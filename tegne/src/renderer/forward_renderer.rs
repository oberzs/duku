// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// ForwardRenderer - renderer that renders shadowmap and then normal render pass

use std::sync::Arc;
use std::time::Instant;

use super::Order;
use super::Target;
use crate::camera::Camera;
use crate::camera::CameraType;
use crate::color::colors;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::math::Matrix4;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::pipeline::ImageUniform;
use crate::pipeline::Light;
use crate::pipeline::PushConstants;
use crate::pipeline::RenderPass;
use crate::pipeline::ShaderLayout;
use crate::pipeline::WorldData;
use crate::profile_scope;
use crate::resource::IdRef;
use crate::resource::ResourceManager;

pub(crate) struct ForwardRenderer {
    depth_framebuffer: Framebuffer,
    start_time: Instant,
}

pub(crate) struct ForwardDrawOptions<'a> {
    pub(crate) framebuffer: &'a Framebuffer,
    pub(crate) color_pass: &'a RenderPass,
    pub(crate) depth_pass: &'a RenderPass,
    pub(crate) shader_layout: &'a ShaderLayout,
    pub(crate) resources: &'a ResourceManager,
    pub(crate) target: Target<'a>,
}

impl ForwardRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        depth_pass: &RenderPass,
        image_uniform: &ImageUniform,
        shader_layout: &ShaderLayout,
    ) -> Result<Self> {
        profile_scope!("new");

        // TODO: no need for multiple depth framebuffers
        // might be a sync issue

        let depth_framebuffer = Framebuffer::depth(
            device,
            depth_pass,
            image_uniform,
            shader_layout,
            CameraType::Orthographic,
            2048,
            2048,
        )?;

        Ok(Self {
            start_time: Instant::now(),
            depth_framebuffer,
        })
    }

    pub(crate) fn draw(&self, device: &Device, options: ForwardDrawOptions<'_>) -> Result<()> {
        let framebuffer = options.framebuffer;
        let clear = options.target.clear();
        let cmd = device.command_buffer();

        let light_dir = Vector3::new(-1.0, -2.0, -1.0).unit();

        let light_mat = if options.target.has_shadows() {
            // frustum-fit light camera
            // get view frustum corners from NDC
            let cam_inv = framebuffer.camera.matrix().inverse().unwrap();
            let mut corners = vec![];
            for x in &[-1.0, 1.0] {
                for y in &[-1.0, 1.0] {
                    for z in &[0.0, 1.0] {
                        let corner = cam_inv * Vector4::new(*x, *y, *z, 1.0);
                        corners.push(corner.shrink() / corner.w);
                    }
                }
            }

            // get bounding sphere radius
            let corner_count = corners.len() as f32;
            let center: Vector3 = corners.iter().sum::<Vector3>() / corner_count;
            let r =
                corners.iter().map(|v| (center - *v).length()).sum::<f32>() / corners.len() as f32;

            // create depth camera
            let light_pos = center - light_dir * r;
            let size = (r * 2.0) as u32;
            let mut depth_cam = Camera::orthographic(size, size);
            depth_cam.depth = size;
            depth_cam.transform.look_in_dir(light_dir, Vector3::up());
            depth_cam.transform.position = light_pos;

            depth_cam.matrix()
        } else {
            Matrix4::identity()
        };

        // setup lights
        let main_light = Light {
            coords: light_dir.extend(0.0),
            color: colors::WHITE.to_rgba_norm_vec(),
        };
        let other_lights = options.target.lights();

        // update world uniform
        let world_data = WorldData {
            shadow_index: self.depth_framebuffer.image_index(),
            lights: [
                main_light,
                other_lights[0],
                other_lights[1],
                other_lights[2],
            ],
            cam_mat: framebuffer.camera.matrix(),
            cam_pos: framebuffer.camera.transform.position,
            light_mat,
            time: self.start_time.elapsed().as_secs_f32(),
        };

        // shadow mapping
        if options.target.has_shadows() {
            device.cmd_begin_render_pass(cmd, &self.depth_framebuffer, options.depth_pass, clear);
            self.setup_pass(device, &self.depth_framebuffer);
            self.bind_world(device, &self.depth_framebuffer, world_data, &options)?;

            self.bind_shader(device, options.resources.builtin("shadow_sh"), &options);
            for s_order in options.target.orders_by_shader() {
                for m_order in s_order.orders_by_material() {
                    self.bind_material(device, m_order.material(), &options)?;
                    for order in m_order.orders() {
                        if order.has_shadows {
                            self.draw_order(device, order, &options)?;
                        }
                    }
                }
            }

            device.cmd_end_render_pass(cmd);
            self.depth_framebuffer.update_shader_image(cmd);
        }

        // normal render
        device.cmd_begin_render_pass(cmd, framebuffer, options.color_pass, clear);
        self.setup_pass(device, framebuffer);
        self.bind_world(device, framebuffer, world_data, &options)?;

        for s_order in options.target.orders_by_shader() {
            self.bind_shader(device, s_order.shader(), &options);
            for m_order in s_order.orders_by_material() {
                self.bind_material(device, m_order.material(), &options)?;
                for order in m_order.orders() {
                    self.draw_order(device, order, &options)?;
                }
            }
        }

        // wireframe render
        self.bind_shader(device, options.resources.builtin("wireframe_sh"), &options);
        for order in options.target.wireframe_orders() {
            self.draw_order(device, order, &options)?;
        }

        device.cmd_end_render_pass(cmd);
        framebuffer.update_shader_image(cmd);

        Ok(())
    }

    fn setup_pass(&self, device: &Device, framebuffer: &Framebuffer) {
        let cmd = device.command_buffer();
        device.cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        device.cmd_set_line_width(cmd, 1.0);
    }

    fn bind_world(
        &self,
        device: &Device,
        framebuffer: &Framebuffer,
        data: WorldData,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        framebuffer.world_uniform().update(data)?;
        device.cmd_bind_descriptor(
            cmd,
            framebuffer.world_uniform().descriptor(),
            options.shader_layout,
        );
        Ok(())
    }

    fn bind_shader(&self, device: &Device, shader: IdRef, options: &ForwardDrawOptions<'_>) {
        let cmd = device.command_buffer();
        let resources = options.resources;
        resources.with_shader(shader, |s| device.cmd_bind_shader(cmd, s));
    }

    fn bind_material(
        &self,
        device: &Device,
        material: IdRef,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        let resources = options.resources;
        if let Some(descriptor) = resources.with_material(material, |m| m.descriptor()) {
            device.cmd_bind_descriptor(cmd, descriptor?, options.shader_layout);
        }
        Ok(())
    }

    fn draw_order(
        &self,
        device: &Device,
        order: Order,
        options: &ForwardDrawOptions<'_>,
    ) -> Result<()> {
        let cmd = device.command_buffer();
        let resources = options.resources;
        let albedo = resources
            .with_texture(order.albedo, |t| t.image_index())
            .or_else(|| resources.with_framebuffer(order.albedo, |f| f.image_index()));
        if let Some(albedo_index) = albedo {
            if let Some((vb, ib, n)) = resources.with_mesh(order.mesh, |m| {
                (m.vertex_buffer(), m.index_buffer(), m.index_count())
            }) {
                device.cmd_push_constants(
                    cmd,
                    PushConstants {
                        model_mat: order.model,
                        albedo_index,
                    },
                    options.shader_layout,
                );
                device.cmd_bind_vertex_buffer(cmd, vb?);
                device.cmd_bind_index_buffer(cmd, ib?);
                device.cmd_draw(cmd, n);
            }
        }
        Ok(())
    }
}
