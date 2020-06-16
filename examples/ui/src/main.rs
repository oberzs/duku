// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use tegne::CameraType;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (500, 500);

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
        if events.is_resized() {
            let (new_width, new_height) = events.size();
            tegne.resize(new_width, new_height);
        }

        ui.show_demo_window(&mut show_demo);
        let ui_data = ui.render();

        tegne.draw_on_window(|target| {
            target.draw_ui(ui_data);
        });
    });
}
