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

use crate::color::Color;
use crate::device::Device;
use crate::device::Stats;
use crate::image::Framebuffer;
use crate::image::ImageFormat;
use crate::image::Msaa;
use crate::image::Size;
use crate::image::Texture;
use crate::math::Matrix4;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::mesh::Mesh;
use crate::pipeline::Shader;
use crate::pipeline::ShaderConstants;
use crate::pipeline::ShaderImages;
use crate::pipeline::ShaderLayout;
use crate::pipeline::ShaderWorld;
use crate::renderer::Camera;
use crate::storage::Handle;
use crate::storage::Storage;

pub use imgui;

pub(crate) struct Ui {
    framebuffer: Handle<Framebuffer>,
    camera: Camera,
    shader: Shader,
    mesh: Mesh,
    texture: Texture,
    drawn: bool,

    imgui: ImContext,
}

pub struct UiFrame<'ui> {
    pub frame: ImUi<'ui>,
}

impl Ui {
    pub(crate) fn new(
        device: &Device,
        shader_layout: &ShaderLayout,
        shader_images: &mut ShaderImages,
        storage: &mut Storage,
        size: Size,
    ) -> Self {
        // create imgui context
        let mut imgui = ImContext::create();
        imgui.set_ini_filename(None);
        {
            // setup imgui backend
            let io = imgui.io_mut();
            io.display_size = [size.width as f32, size.height as f32];
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

        // create ui storage
        let texture = {
            let mut fonts = imgui.fonts();
            fonts.add_font(&[FontSource::DefaultFontData {
                config: Some(FontConfig {
                    size_pixels: 13.0,
                    ..Default::default()
                }),
            }]);
            let ui_texture = fonts.build_alpha8_texture();
            Texture::new(
                device,
                shader_images,
                ui_texture.data.to_vec(),
                Size::new(ui_texture.width, ui_texture.height),
                ImageFormat::Gray,
            )
        };

        let framebuffer = Framebuffer::new(
            device,
            shader_layout,
            shader_images,
            &[ImageFormat::Sbgra],
            Msaa::Disabled,
            size,
        );

        let camera = Camera::orthographic(size.width as f32, size.height as f32);

        let shader = Shader::from_spirv_bytes(
            device,
            &framebuffer,
            shader_layout,
            include_bytes!("../shaders/ui.spirv"),
        )
        .expect("bad shader");

        let framebuffer_handle = storage.add_framebuffer(framebuffer);

        let mesh = Mesh::new(device);

        Self {
            drawn: false,
            framebuffer: framebuffer_handle,
            camera,
            texture,
            shader,
            mesh,
            imgui,
        }
    }

    pub(crate) fn draw(
        &mut self,
        device: &Device,
        shader_layout: &ShaderLayout,
        storage: &mut Storage,
        mut draw_fn: impl FnMut(&UiFrame<'_>),
    ) {
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
                let color = Color::rgba(vert.col[0], vert.col[1], vert.col[2], vert.col[3]);
                vertices.push(vertex);
                uvs.push(uv);
                colors.push(color);
                normals.push(Vector3::BACKWARD);
            }
            to = vertices.len() as u16;
        }
        let textures = vec![self.texture.shader_index(); vertices.len()];

        // update mesh
        self.mesh.set_indices(indices);
        self.mesh.set_vertices(vertices);
        self.mesh.set_normals(normals);
        self.mesh.set_colors(colors);
        self.mesh.set_uvs(uvs);
        self.mesh.set_textures(textures);
        self.mesh.update_if_needed(device);

        // render ui
        let cmd = device.commands();
        let framebuffer = storage.framebuffers.get_mut(&self.framebuffer);

        // update world uniform
        framebuffer.update_world(
            device,
            ShaderWorld {
                world_to_shadow: [Matrix4::identity(); 4],
                world_to_view: self.camera.world_to_view(),
                view_to_clip: self.camera.view_to_clip(),
                camera_position: self.camera.transform.position,
                lights: [Default::default(); 4],
                shadow_cascades: [0.0; 4],
                shadow_bias: 0.0,
                shadow_pcf: 0.0,
                time: 0.0,
            },
        );

        // begin render pass
        cmd.begin_render_pass(framebuffer, (0.0, 0.0, 0.0, 0.0));
        cmd.set_view(framebuffer.size());

        // bind storage
        cmd.bind_descriptor(shader_layout, framebuffer.world());
        cmd.bind_shader(&self.shader);

        // render mesh
        cmd.push_constants(
            shader_layout,
            ShaderConstants {
                local_to_world: Matrix4::identity(),
                sampler_index: 0,
            },
        );

        cmd.bind_mesh(&self.mesh);
        cmd.draw(self.mesh.index_count(), 0);

        cmd.end_render_pass();
        framebuffer.blit_to_texture(cmd);

        self.drawn = true;
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
        device: &Device,
        storage: &mut Storage,
        shader_images: &mut ShaderImages,
        size: Size,
    ) {
        self.imgui.io_mut().display_size = [size.width as f32, size.height as f32];
        self.camera.width = size.width as f32;
        self.camera.height = size.height as f32;

        let framebuffer = storage.framebuffers.get_mut(&self.framebuffer);
        framebuffer.resize(size.width, size.height);
        framebuffer.update_if_needed(device, shader_images);
    }

    pub(crate) fn destroy(&self, device: &Device) {
        self.texture.destroy(device);
        self.mesh.destroy(device);
        self.shader.destroy(device);
    }

    pub(crate) const fn drawn(&self) -> bool {
        self.drawn
    }

    pub(crate) fn reset(&mut self) {
        self.drawn = false;
    }

    // pub(crate) const fn framebuffer(&self) -> &Framebuffer {
    //     &self.framebuffer
    // }
}

impl UiFrame<'_> {
    pub fn text(&self, text: impl AsRef<str>) {
        self.frame.text(text);
    }

    pub fn color_edit(&self, label: impl AsRef<str>, color: &mut Color) {
        let cstring = CString::new(label.as_ref()).expect("bad cstring");
        let im_label = unsafe { ImStr::from_cstr_unchecked(&cstring) };

        let norm = color.to_rgba_norm();
        let mut color_array = [norm.0, norm.1, norm.2, norm.3];
        ColorEdit::new(im_label, &mut color_array).build(&self.frame);
        *color = (
            color_array[0],
            color_array[1],
            color_array[2],
            color_array[3],
        )
            .into();
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

    pub fn stats_window(&self, stats: Stats, fps: u32, delta_time: f32) {
        let pad = 14;

        let fps = format!("{1:0$} : {2}", pad, "Fps", fps);
        let frame_time = format!("{1:0$} : {2:.2}ms", pad, "Frame Time", delta_time * 1000.0);
        let drawn_indices = format!(
            "{1:0$} : {2}({3})",
            pad,
            "Drawn Indices",
            stats.drawn_indices,
            stats.drawn_indices / 3
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
                self.frame.separator();
                self.frame.text(drawn_indices);
                self.frame.text(draw_calls);
                self.frame.separator();
                self.frame.text(shader_rebinds);
                self.frame.text(material_rebinds);
            });
    }
}
