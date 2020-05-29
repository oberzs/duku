// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use tegne::ui;
use tegne::ui::im_str;
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

    let mut color = [0.0, 0.0, 0.0];

    window.main_loop(|events, ui| {
        if events.is_resized() {
            let (new_width, new_height) = events.size();
            tegne.resize(new_width, new_height);
        }

        // TODO: multiple window support
        ui::Window::new(im_str!("Background control"))
            .size([300.0, 300.0], ui::Condition::FirstUseEver)
            .build(&ui, || {
                ui::ColorPicker::new(im_str!("color"), &mut color).build(&ui);
            });
        let ui_data = ui.render();

        tegne.draw_on_window(|target| {
            target.set_clear(color);
            target.draw_ui(ui_data);
        });
    });
}
