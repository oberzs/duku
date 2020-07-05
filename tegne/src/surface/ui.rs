#![cfg(feature = "ui")]

use imgui::Ui;

use crate::surface::Events;
use crate::Tegne;

pub use imgui::im_str as ui_str;
pub use imgui::ColorPicker;
pub use imgui::Condition;
pub use imgui::Window;

pub fn stats_window(ui: &Ui<'_>, tegne: &Tegne, events: &Events) {
    let render_stats = tegne.render_stats();
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

    let [display_width, _] = ui.io().display_size;

    Window::new(ui_str!("Stats"))
        .position([display_width - (180.0 + 10.0), 10.0], Condition::Always)
        .size([1.0, 1.0], Condition::FirstUseEver)
        .always_auto_resize(true)
        .resizable(false)
        .movable(false)
        .title_bar(false)
        .build(&ui, || {
            ui.text(fps);
            ui.text(frame_time);
            ui.text(total_time);
            ui.text(drawn_indices);
            ui.text(draw_calls);
            ui.text(shader_rebinds);
            ui.text(material_rebinds);
        });
}
