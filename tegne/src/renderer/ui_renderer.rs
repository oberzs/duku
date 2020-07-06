#![cfg(feature = "ui")]

use imgui::Ui;
use std::sync::Arc;

use crate::camera::CameraType;
use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::Texture;
use crate::image::TextureFormat;
use crate::image::TextureOptions;
use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::AttachmentType;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::pipeline::WorldData;
use crate::resource::Ref;
use crate::resource::ResourceManager;

pub(crate) struct UiRenderer {
    framebuffer: Ref<Framebuffer>,
    shader: Shader,
    mesh: Mesh,
    texture: Option<Texture>,
    device: Arc<Device>,
}

impl UiRenderer {
    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        resources: &ResourceManager,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        profile_scope!("new");

        let framebuffer = Framebuffer::new(
            device,
            shader_layout,
            FramebufferOptions {
                attachment_types: &[AttachmentType::Color],
                camera_type: CameraType::Orthographic,
                multisampled: false,
                width,
                height,
            },
        )?;

        let shader = Shader::new(
            device,
            &framebuffer,
            shader_layout,
            include_bytes!("../../assets/shaders/ui.shader"),
            ShaderOptions {
                depth_test: false,
                ..Default::default()
            },
        )?;

        let mesh = Mesh::new(
            device,
            MeshOptions {
                vertices: &[Vector3::new(0.0, 0.0, 0.0)],
                triangles: &[[0, 0, 0]],
                ..Default::default()
            },
        )?;

        Ok(Self {
            device: Arc::clone(device),
            framebuffer: resources.add_framebuffer(framebuffer),
            texture: None,
            shader,
            mesh,
        })
    }

    pub(crate) fn draw(&mut self, ui: Ui<'_>, shader_layout: &ShaderLayout) -> Result<()> {
        let draw_data = ui.render();

        let half_width = draw_data.display_size[0] / 2.0;
        let half_height = draw_data.display_size[1] / 2.0;

        // generate mesh data
        let mut triangles = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
        let mut uvs = vec![];
        let mut to = 0;
        for draw_list in draw_data.draw_lists() {
            for tri in draw_list.idx_buffer().chunks(3) {
                triangles.push([tri[0] as u32 + to, tri[1] as u32 + to, tri[2] as u32 + to]);
            }
            for vert in draw_list.vtx_buffer() {
                let vertex =
                    Vector3::new(vert.pos[0] - half_width, -vert.pos[1] + half_height, 1.0);
                let uv = Vector2::new(vert.uv[0], vert.uv[1]);
                let color = Color::from(vert.col);
                vertices.push(vertex);
                uvs.push(uv);
                colors.push(color);
                normals.push(Vector3::backward());
            }
            to = vertices.len() as u32;
        }

        // update mesh
        self.mesh.set_vertices(&vertices);
        self.mesh.set_normals(&normals);
        self.mesh.set_colors(&colors);
        self.mesh.set_uvs(&uvs);
        self.mesh.set_triangles(&triangles);

        // render ui
        let cmd = self.device.command_buffer();

        self.framebuffer.with(|f| {
            // update world uniform
            f.world_uniform()
                .update(WorldData {
                    lights: [Default::default(); 4],
                    world_matrix: f.camera.matrix(),
                    camera_position: f.camera.transform.position,
                    time: 0.0,
                    cascade_splits: [0.0; 4],
                    light_matrices: [Matrix4::identity(); 4],
                    bias: 0.0,
                })
                .expect("bad code");

            // begin render pass
            self.device
                .cmd_begin_render_pass(cmd, &f, [0.0, 0.0, 0.0, 0.0]);
            self.device.cmd_set_view(cmd, f.width(), f.height());
            self.device.cmd_set_line_width(cmd, 1.0);

            // bind resources
            self.device
                .cmd_bind_descriptor(cmd, f.world_uniform().descriptor(), shader_layout);
            self.device.cmd_bind_shader(cmd, &self.shader);

            // render mesh
            let albedo_index = if let Some(texture) = &self.texture {
                texture.image_index()
            } else {
                0
            };
            self.device.cmd_push_constants(
                cmd,
                PushConstants {
                    model_matrix: Matrix4::identity(),
                    sampler_index: 0,
                    albedo_index,
                },
                shader_layout,
            );
            self.device
                .cmd_bind_vertex_buffer(cmd, self.mesh.vertex_buffer().expect("bad code"));
            self.device
                .cmd_bind_index_buffer(cmd, self.mesh.index_buffer().expect("bad code"));
            self.device.cmd_draw(cmd, self.mesh.index_count());

            self.device.cmd_end_render_pass(cmd);
        });

        Ok(())
    }

    pub(crate) fn set_font_texture(
        &mut self,
        image_uniform: &ImageUniform,
        texture: (Vec<u8>, u32, u32),
    ) -> Result<()> {
        self.texture = Some(Texture::new(
            &self.device,
            image_uniform,
            TextureOptions {
                data: &texture.0,
                width: texture.1,
                height: texture.2,
                format: TextureFormat::Rgba,
            },
        )?);

        Ok(())
    }

    pub(crate) fn resize(&self, shader_layout: &ShaderLayout, width: u32, height: u32) {
        self.framebuffer
            .with(|f| f.resize(width, height, shader_layout).expect("bad code"));
    }

    pub(crate) fn framebuffer(&self) -> &Ref<Framebuffer> {
        &self.framebuffer
    }
}
