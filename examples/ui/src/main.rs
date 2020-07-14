// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use tegne::ui;
use tegne::ui::label;
use tegne::CameraType;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (mut width, mut height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: UI",
        resizable: true,
        width,
        height,
    });
    let mut tegne = Tegne::from_window(
        &mut window,
        TegneOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    );

    let mut show_demo = true;

    window.main_loop(|events, ui| {
        if let Some((new_width, new_height)) = events.resized() {
            tegne.resize(new_width, new_height);
            width = new_width;
            height = new_height;
        }

        // ui.show_demo_window(&mut show_demo);
        ui::Window::new(label!("window"))
            .size([100.0, 100.0], ui::Condition::FirstUseEver)
            .build(&ui, || {
                ui.separator();
            });

        tegne.draw_ui(ui);
        tegne.draw_on_window(|_| {});
    });
}
