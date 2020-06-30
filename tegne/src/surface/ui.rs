#![cfg(feature = "ui")]

use imgui::Ui;

use crate::surface::Events;
use crate::Tegne;

pub use imgui::im_str as ui_str;
pub use imgui::ColorPicker;
pub use imgui::Condition;
pub use imgui::Window;

pub fn stats_window(ui: &Ui<'_>, tegne: &Tegne, events: &Events) {
    let fps = format!("{:10} : {}", "Fps", events.fps());
    let frame_time = format!(
        "{:10} : {:.2}ms",
        "Frame Time",
        events.delta_time() * 1000.0
    );
    let total_time = format!("{:10} : {:.2}s", "Total Time", tegne.time());

    Window::new(ui_str!("Stats"))
        .position([10.0, 10.0], Condition::FirstUseEver)
        .size([1.0, 1.0], Condition::FirstUseEver)
        .always_auto_resize(true)
        .resizable(false)
        .movable(false)
        .title_bar(false)
        .build(&ui, || {
            ui.text(fps);
            ui.text(frame_time);
            ui.text(total_time);
        });
}
