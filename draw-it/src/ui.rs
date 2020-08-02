#![cfg(feature = "ui")]

use glfw::Action;
use glfw::Key;
use glfw::Modifiers;
use glfw::MouseButton;
use glfw::WindowEvent;
use imgui::BackendFlags;
use imgui::ColorEdit;
use imgui::Context as ImContext;
use imgui::FontConfig;
use imgui::FontSource;
use imgui::ImStr;
use imgui::Key as ImKey;
use imgui::Ui as ImUi;
use std::sync::Arc;

use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::ImageFormat;
use crate::image::Texture;
use crate::image::TextureOptions;
use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::mesh::Mesh;
use crate::mesh::MeshOptions;
use crate::pipeline::CullMode;
use crate::pipeline::DepthMode;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::Shader;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderOptions;
use crate::pipeline::WorldData;
use crate::renderer::CameraType;
use crate::resource::Ref;
use crate::resource::ResourceManager;
use crate::stats::Stats;

pub use imgui::im_str as label;
pub use imgui::ColorPicker;
pub use imgui::Condition;
pub use imgui::Slider;
pub use imgui::Window;

pub(crate) struct Ui {
    framebuffer: Ref<Framebuffer>,
    shader: Shader,
    mesh: Mesh,
    texture: Texture,
    drawn: bool,

    imgui: ImContext,

    device: Arc<Device>,
}

impl Ui {
    pub(crate) fn new(
        device: &Arc<Device>,
        shader_layout: &ShaderLayout,
        image_uniform: &mut ImageUniform,
        resources: &mut ResourceManager,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        // create imgui context
        let mut imgui = ImContext::create();
        imgui.set_ini_filename(None);
        {
            // setup imgui backend
            let io = imgui.io_mut();
            io.display_size = [width as f32, height as f32];
            io.display_framebuffer_scale = [1.0, 1.0];
            io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
            io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);
            io[ImKey::Tab] = Key::Tab as _;
            io[ImKey::LeftArrow] = Key::Left as _;
            io[ImKey::RightArrow] = Key::Right as _;
            io[ImKey::UpArrow] = Key::Up as _;
            io[ImKey::DownArrow] = Key::Down as _;
            io[ImKey::PageUp] = Key::PageUp as _;
            io[ImKey::PageDown] = Key::PageDown as _;
            io[ImKey::Home] = Key::Home as _;
            io[ImKey::End] = Key::End as _;
            io[ImKey::Insert] = Key::Insert as _;
            io[ImKey::Delete] = Key::Delete as _;
            io[ImKey::Backspace] = Key::Backspace as _;
            io[ImKey::Space] = Key::Space as _;
            io[ImKey::Enter] = Key::Enter as _;
            io[ImKey::Escape] = Key::Escape as _;
            io[ImKey::KeyPadEnter] = Key::KpEnter as _;
            io[ImKey::A] = Key::A as _;
            io[ImKey::C] = Key::C as _;
            io[ImKey::V] = Key::V as _;
            io[ImKey::X] = Key::X as _;
            io[ImKey::Y] = Key::Y as _;
            io[ImKey::Z] = Key::Z as _;
        }
        {
            let mut style = imgui.style_mut();
            style.window_rounding = 3.0;
            style.use_dark_colors();
        }

        // create ui resources
        let texture = {
            let mut fonts = imgui.fonts();
            fonts.add_font(&[FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: 13.0,
                    ..Default::default()
                }),
            }]);
            // TODO: change to alpha8 texture
            let ui_texture = fonts.build_alpha8_texture();
            Texture::new(
                device,
                image_uniform,
                TextureOptions {
                    format: ImageFormat::Gray,
                    data: ui_texture.data.to_vec(),
                    width: ui_texture.width,
                    height: ui_texture.height,
                },
            )?
        };

        let framebuffer = Framebuffer::new(
            device,
            shader_layout,
            image_uniform,
            FramebufferOptions {
                attachment_formats: &[ImageFormat::Sbgra],
                camera_type: CameraType::Orthographic,
                multisampled: false,
                depth: false,
                width,
                height,
            },
        )?;

        let shader = Shader::new(
            device,
            &framebuffer,
            shader_layout,
            include_bytes!("../shaders/ui.shader"),
            ShaderOptions {
                depth_mode: DepthMode::Disabled,
                cull_mode: CullMode::Disabled,
                ..Default::default()
            },
        )?;

        let mesh = Mesh::new(
            device,
            MeshOptions {
                vertices: &[Vector3::new(0.0, 0.0, 0.0)],
                indices: &[0, 0, 0],
                ..Default::default()
            },
        )?;

        Ok(Self {
            device: Arc::clone(device),
            framebuffer: resources.add_framebuffer(framebuffer),
            drawn: false,
            texture,
            shader,
            mesh,
            imgui,
        })
    }

    pub(crate) fn draw(
        &mut self,
        shader_layout: &ShaderLayout,
        mut draw_fn: impl FnMut(&imgui::Ui<'_>),
    ) -> Result<()> {
        let ui = self.imgui.frame();
        draw_fn(&ui);
        let draw_data = ui.render();

        let half_width = draw_data.display_size[0] / 2.0;
        let half_height = draw_data.display_size[1] / 2.0;

        // generate mesh data
        let mut indices = vec![];
        let mut vertices = vec![];
        let mut normals = vec![];
        let mut colors = vec![];
        let mut uvs = vec![];
        let mut to = 0;
        for draw_list in draw_data.draw_lists() {
            indices.extend(draw_list.idx_buffer().iter().map(|i| *i as u32 + to));
            for vert in draw_list.vtx_buffer() {
                let vertex =
                    Vector3::new(vert.pos[0] - half_width, -vert.pos[1] + half_height, 1.0);
                let uv = Vector2::new(vert.uv[0], vert.uv[1]);
                let color = Color::from(vert.col);
                vertices.push(vertex);
                uvs.push(uv);
                colors.push(color);
                normals.push(Vector3::BACKWARD);
            }
            to = vertices.len() as u32;
        }

        // update mesh
        self.mesh.set_vertices(&vertices);
        self.mesh.set_normals(&normals);
        self.mesh.set_colors(&colors);
        self.mesh.set_uvs(&uvs);
        self.mesh.set_indices(&indices);
        self.mesh.update_if_needed()?;

        // render ui
        let cmd = self.device.command_buffer();

        self.framebuffer.with(|f| {
            // update world uniform
            let world_matrix = f.camera.matrix();
            let camera_position = f.camera.transform.position;
            f.world_uniform
                .update(WorldData {
                    light_matrices: [Matrix4::identity(); 4],
                    lights: [Default::default(); 4],
                    cascade_splits: [0.0; 4],
                    time: 0.0,
                    pcf: 0.0,
                    camera_position,
                    world_matrix,
                })
                .expect("bad update");

            // begin render pass
            self.device
                .cmd_begin_render_pass(cmd, &f, [0.0, 0.0, 0.0, 0.0]);
            self.device.cmd_set_view(cmd, f.width(), f.height());
            self.device.cmd_set_line_width(cmd, 1.0);

            // bind resources
            self.device
                .cmd_bind_uniform(cmd, shader_layout, &f.world_uniform);
            self.device.cmd_bind_shader(cmd, &self.shader);

            // render mesh
            self.device.cmd_push_constants(
                cmd,
                shader_layout,
                PushConstants {
                    albedo_index: self.texture.image_index(),
                    model_matrix: Matrix4::identity(),
                    sampler_index: 0,
                },
            );

            self.device.cmd_bind_mesh(cmd, &self.mesh);
            self.device.cmd_draw(cmd, self.mesh.index_count(), 0);

            self.device.cmd_end_render_pass(cmd);
            f.blit_to_texture(cmd);
        });

        self.drawn = true;

        Ok(())
    }

    pub(crate) fn handle_event(&mut self, event: &WindowEvent) {
        let io = self.imgui.io_mut();

        match event {
            WindowEvent::Key(key, _, action, modifiers) => {
                // modifiers
                io.key_shift = modifiers.contains(Modifiers::Shift);
                io.key_ctrl = modifiers.contains(Modifiers::Control);
                io.key_alt = modifiers.contains(Modifiers::Alt);
                io.key_super = modifiers.contains(Modifiers::Super);

                // action
                match action {
                    Action::Release => io.keys_down[*key as usize] = false,
                    Action::Press => io.keys_down[*key as usize] = true,
                    _ => (),
                }
            }
            WindowEvent::CursorPos(x, y) => {
                io.mouse_pos = [*x as f32, *y as f32];
            }
            WindowEvent::Char(ch) if *ch != '\u{7f}' => io.add_input_character(*ch),
            WindowEvent::Scroll(x, y) => {
                io.mouse_wheel_h = *x as f32;
                io.mouse_wheel = *y as f32;
            }
            WindowEvent::MouseButton(button, action, _) => {
                let pressed = matches!(action, Action::Press);
                match button {
                    MouseButton::Button1 => io.mouse_down[0] = pressed,
                    MouseButton::Button2 => io.mouse_down[1] = pressed,
                    MouseButton::Button3 => io.mouse_down[2] = pressed,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    pub(crate) fn resize(
        &mut self,
        uniform: &mut ImageUniform,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.imgui.io_mut().display_size = [width as f32, height as f32];
        self.framebuffer.with(|f| f.resize(width, height, uniform))
    }

    pub(crate) fn drawn(&self) -> bool {
        self.drawn
    }

    pub(crate) fn reset(&mut self) {
        self.drawn = false;
    }

    pub(crate) fn framebuffer(&self) -> &Ref<Framebuffer> {
        &self.framebuffer
    }
}

pub fn color_edit(ui: &ImUi<'_>, label: &ImStr, color: &mut Color) {
    let mut color_array = color.to_rgba_norm();
    ColorEdit::new(label, &mut color_array).build(ui);
    *color = color_array.into();
}

pub fn drag_vector2(ui: &ImUi<'_>, label: &ImStr, vector: &mut Vector2) {
    let mut floats = [vector.x, vector.y];
    ui.drag_float2(label, &mut floats).build();
    vector.x = floats[0];
    vector.y = floats[1];
}

pub fn drag_vector3(ui: &ImUi<'_>, label: &ImStr, vector: &mut Vector3) {
    let mut floats = [vector.x, vector.y, vector.z];
    ui.drag_float3(label, &mut floats).build();
    vector.x = floats[0];
    vector.y = floats[1];
    vector.z = floats[2];
}

pub fn drag_vector4(ui: &ImUi<'_>, label: &ImStr, vector: &mut Vector4) {
    let mut floats = [vector.x, vector.y, vector.z, vector.w];
    ui.drag_float4(label, &mut floats).build();
    vector.x = floats[0];
    vector.y = floats[1];
    vector.z = floats[2];
    vector.w = floats[3];
}

pub fn stats_window(ui: &ImUi<'_>, stats: Stats) {
    let pad = 14;

    let fps = format!("{1:0$} : {2}", pad, "Fps", stats.fps);
    let frame_time = format!(
        "{1:0$} : {2:.2}ms",
        pad,
        "Frame Time",
        stats.delta_time * 1000.0
    );
    let total_time = format!("{1:0$} : {2:.2}s", pad, "Total Time", stats.time);
    let drawn_indices = format!(
        "{1:0$} : {2}({3})",
        pad,
        "Drawn Indices",
        stats.drawn_indices,
        stats.drawn_triangles()
    );
    let shader_rebinds = format!(
        "{1:0$} : {2}({3})",
        pad, "Shaders Used", stats.shaders_used, stats.shader_rebinds
    );
    let material_rebinds = format!(
        "{1:0$} : {2}({3})",
        pad, "Materials Used", stats.materials_used, stats.material_rebinds
    );
    let draw_calls = format!("{1:0$} : {2}", pad, "Draw Calls", stats.draw_calls);

    Window::new(label!("Stats"))
        .position([10.0, 10.0], Condition::Always)
        .size([1.0, 1.0], Condition::FirstUseEver)
        .always_auto_resize(true)
        .resizable(false)
        .movable(false)
        .title_bar(false)
        .build(&ui, || {
            ui.text(fps);
            ui.text(frame_time);
            ui.text(total_time);
            ui.separator();
            ui.text(drawn_indices);
            ui.text(draw_calls);
            ui.separator();
            ui.text(shader_rebinds);
            ui.text(material_rebinds);
        });
}
