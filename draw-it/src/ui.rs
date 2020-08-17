// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Imgui abstraction for use in draw-it

#![cfg(feature = "ui")]

use glfw::Action;
use glfw::Key;
use glfw::Modifiers;
use glfw::MouseButton;
use glfw::WindowEvent;
use imgui::im_str;
use imgui::BackendFlags;
use imgui::ColorEdit;
use imgui::Condition;
use imgui::Context as ImContext;
use imgui::FontConfig;
use imgui::FontSource;
use imgui::ImStr;
use imgui::Key as ImKey;
use imgui::Slider;
use imgui::Ui as ImUi;
use imgui::Window;
use std::ffi::CString;
use std::sync::Arc;

use crate::color::Color;
use crate::device::Device;
use crate::error::Result;
use crate::image::CoreFramebuffer;
use crate::image::CoreTexture;
use crate::image::Framebuffer;
use crate::image::FramebufferOptions;
use crate::image::FramebufferUpdateData;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::image::TextureOptions;
use crate::image::WorldUpdateData;
use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::mesh::CoreMesh;
use crate::mesh::MeshUpdateData;
use crate::pipeline::CoreShader;
use crate::pipeline::ImageUniform;
use crate::pipeline::PushConstants;
use crate::pipeline::ShaderLayout;
use crate::renderer::CameraType;
use crate::resource::ResourceManager;
use crate::stats::Stats;

pub use imgui;

pub(crate) struct Ui {
    framebuffer: Framebuffer,
    shader: CoreShader,
    mesh: CoreMesh,
    texture: CoreTexture,
    drawn: bool,

    imgui: ImContext,

    device: Arc<Device>,
}

pub struct UiFrame<'ui> {
    pub frame: ImUi<'ui>,
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
            let ui_texture = fonts.build_alpha8_texture();
            CoreTexture::new(
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

        let core_framebuffer = CoreFramebuffer::new(
            device,
            shader_layout,
            image_uniform,
            FramebufferOptions {
                attachment_formats: &[ImageFormat::Sbgra],
                camera_type: CameraType::Orthographic,
                msaa: Msaa::Disabled,
                depth: false,
                width,
                height,
            },
        )?;

        let shader = CoreShader::new(
            device,
            &core_framebuffer,
            shader_layout,
            include_bytes!("../shaders/ui.shader"),
        )?;

        let (index, updater) = resources.framebuffers.add(core_framebuffer);
        let mut framebuffer = Framebuffer::new(index, updater);
        framebuffer.width = width;
        framebuffer.height = height;

        let mesh = CoreMesh::new(device)?;

        Ok(Self {
            device: Arc::clone(device),
            drawn: false,
            framebuffer,
            texture,
            shader,
            mesh,
            imgui,
        })
    }

    pub(crate) fn draw(
        &mut self,
        shader_layout: &ShaderLayout,
        resources: &mut ResourceManager,
        mut draw_fn: impl FnMut(&UiFrame<'_>),
    ) -> Result<()> {
        let draw_data = {
            let ui_frame = UiFrame {
                frame: self.imgui.frame(),
            };
            draw_fn(&ui_frame);
            ui_frame.frame.render()
        };

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
            indices.extend(draw_list.idx_buffer().iter().map(|i| *i + to));
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
            to = vertices.len() as u16;
        }

        // update mesh
        self.mesh.update(MeshUpdateData {
            vertices,
            normals,
            colors,
            uvs,
            indices,
        })?;

        // render ui
        let cmd = self.device.command_buffer();
        let framebuffer = resources.framebuffers.get_mut(&self.framebuffer.index);

        // update world uniform
        let world_matrix = framebuffer.camera.matrix();
        let camera_position = framebuffer.camera.transform.position;
        framebuffer.world_buffer().update_data(&[WorldUpdateData {
            light_matrices: [Matrix4::identity(); 4],
            lights: [Default::default(); 4],
            cascade_splits: [0.0; 4],
            bias: 0.0,
            time: 0.0,
            pcf: 0.0,
            camera_position,
            world_matrix,
        }])?;

        // begin render pass
        self.device
            .cmd_begin_render_pass(cmd, framebuffer, [0.0, 0.0, 0.0, 0.0]);
        self.device
            .cmd_set_view(cmd, framebuffer.width(), framebuffer.height());
        self.device.cmd_set_line_width(cmd, 1.0);

        // bind resources
        self.device
            .cmd_bind_descriptor(cmd, shader_layout, framebuffer.world_descriptor());
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
        framebuffer.blit_to_texture(cmd);

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
        resources: &mut ResourceManager,
        image_uniform: &mut ImageUniform,
        width: u32,
        height: u32,
    ) -> Result<()> {
        self.imgui.io_mut().display_size = [width as f32, height as f32];
        self.framebuffer.width = width;
        self.framebuffer.height = height;
        resources
            .framebuffers
            .get_mut(&self.framebuffer.index)
            .update(image_uniform, FramebufferUpdateData { width, height })
    }

    pub(crate) fn drawn(&self) -> bool {
        self.drawn
    }

    pub(crate) fn reset(&mut self) {
        self.drawn = false;
    }

    pub(crate) fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }
}

impl UiFrame<'_> {
    pub fn text(&self, text: impl AsRef<str>) {
        self.frame.text(text);
    }

    pub fn color_edit(&self, label: impl AsRef<str>, color: &mut Color) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        let mut color_array = color.to_rgba_norm();
        ColorEdit::new(im_label, &mut color_array).build(&self.frame);
        *color = color_array.into();
    }

    pub fn drag_vector2(&self, label: impl AsRef<str>, vector: &mut Vector2) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        let mut floats = [vector.x, vector.y];
        self.frame.drag_float2(im_label, &mut floats).build();
        vector.x = floats[0];
        vector.y = floats[1];
    }

    pub fn drag_vector3(&self, label: impl AsRef<str>, vector: &mut Vector3) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        let mut floats = [vector.x, vector.y, vector.z];
        self.frame.drag_float3(im_label, &mut floats).build();
        vector.x = floats[0];
        vector.y = floats[1];
        vector.z = floats[2];
    }

    pub fn drag_vector4(&self, label: impl AsRef<str>, vector: &mut Vector4) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        let mut floats = [vector.x, vector.y, vector.z, vector.w];
        self.frame.drag_float4(im_label, &mut floats).build();
        vector.x = floats[0];
        vector.y = floats[1];
        vector.z = floats[2];
        vector.w = floats[3];
    }

    pub fn slider(
        &self,
        label: impl AsRef<str>,
        range: core::ops::RangeInclusive<i32>,
        value: &mut i32,
    ) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        Slider::new(im_label, range).build(&self.frame, value);
    }

    pub fn auto_window(&self, label: impl AsRef<str>, build_fn: impl FnMut()) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        Window::new(im_label)
            .size([1.0, 1.0], Condition::FirstUseEver)
            .always_auto_resize(true)
            .build(&self.frame, build_fn);
    }

    pub fn stats_window(&self, stats: Stats) {
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

        Window::new(im_str!("Stats"))
            .position([10.0, 10.0], Condition::Always)
            .size([1.0, 1.0], Condition::FirstUseEver)
            .always_auto_resize(true)
            .resizable(false)
            .movable(false)
            .title_bar(false)
            .build(&self.frame, || {
                self.frame.text(fps);
                self.frame.text(frame_time);
                self.frame.text(total_time);
                self.frame.separator();
                self.frame.text(drawn_indices);
                self.frame.text(draw_calls);
                self.frame.separator();
                self.frame.text(shader_rebinds);
                self.frame.text(material_rebinds);
            });
    }
}
