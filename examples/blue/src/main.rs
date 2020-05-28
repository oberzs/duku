// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// "Hello, World!" example to open a blue resizable window

use tegne::ui;
use tegne::ui::im_str;
use tegne::Camera;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Blue",
        width,
        height,
        resizable: true,
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());
    let mut camera = Camera::orthographic(width, height);

    window.main_loop(|events, ui| {
        if events.is_resized() {
            let (new_width, new_height) = events.size();
            tegne.resize(new_width, new_height);
            camera.resize(new_width, new_height);
        }

        ui::Window::new(im_str!("UI window"))
            .size([300.0, 100.0], ui::Condition::FirstUseEver)
            .build(&ui, || {
                ui.text("Hello, World!");
            });
        let ui_data = ui.render();

        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.draw_ui(ui_data);
        });
        tegne.end_draw();
    });
}
