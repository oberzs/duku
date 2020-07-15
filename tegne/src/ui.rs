#![cfg(feature = "ui")]

use imgui::ColorEdit;
use imgui::ImStr;
use imgui::Ui;

use crate::color::Color;
use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;
use crate::window::Events;
use crate::Context;

pub use imgui::im_str as label;
pub use imgui::ColorPicker;
pub use imgui::Condition;
pub use imgui::Slider;
pub use imgui::Window;

pub fn color_edit(ui: &Ui<'_>, label: &ImStr, color: &mut Color) {
    let mut color_array = color.to_rgba_norm();
    ColorEdit::new(label, &mut color_array).build(ui);
    *color = color_array.into();
}

pub fn drag_vector2(ui: &Ui<'_>, label: &ImStr, vector: &mut Vector2) {
    let mut floats = [vector.x, vector.y];
    ui.drag_float2(label, &mut floats).build();
    vector.x = floats[0];
    vector.y = floats[1];
}

pub fn drag_vector3(ui: &Ui<'_>, label: &ImStr, vector: &mut Vector3) {
    let mut floats = [vector.x, vector.y, vector.z];
    ui.drag_float3(label, &mut floats).build();
    vector.x = floats[0];
    vector.y = floats[1];
    vector.z = floats[2];
}

pub fn drag_vector4(ui: &Ui<'_>, label: &ImStr, vector: &mut Vector4) {
    let mut floats = [vector.x, vector.y, vector.z, vector.w];
    ui.drag_float4(label, &mut floats).build();
    vector.x = floats[0];
    vector.y = floats[1];
    vector.z = floats[2];
    vector.w = floats[3];
}

pub fn stats_window(ui: &Ui<'_>, context: &Context, events: &Events) {
    let render_stats = context.render_stats();
    let pad = 14;

    let fps = format!("{1:0$} : {2}", pad, "Fps", events.fps());
    let frame_time = format!(
        "{1:0$} : {2:.2}ms",
        pad,
        "Frame Time",
        events.delta_time() * 1000.0
    );
    let total_time = format!("{1:0$} : {2:.2}s", pad, "Total Time", render_stats.time);
    let drawn_indices = format!(
        "{1:0$} : {2}({3})",
        pad, "Drawn Indices", render_stats.drawn_indices, render_stats.drawn_triangles
    );
    let shader_rebinds = format!(
        "{1:0$} : {2}",
        pad, "Shaders Used", render_stats.shaders_used
    );
    let material_rebinds = format!(
        "{1:0$} : {2}",
        pad, "Materials Used", render_stats.materials_used
    );
    let draw_calls = format!("{1:0$} : {2}", pad, "Draw Calls", render_stats.draw_calls);

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
