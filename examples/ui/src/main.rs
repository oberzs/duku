// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use tegne::ui;
use tegne::ui::im_str;
use tegne::Camera;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: UI",
        width,
        height,
        resizable: true,
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());
    let mut camera = Camera::orthographic(width, height);

    let mut light = 10;

    window.main_loop(|events, ui| {
        if events.is_resized() {
            let (new_width, new_height) = events.size();
            tegne.resize(new_width, new_height);
            camera.resize(new_width, new_height);
        }

        // TODO: multiple window support
        ui::Window::new(im_str!("Light control"))
            .size([280.0, 70.0], ui::Condition::FirstUseEver)
            .build(&ui, || {
                ui::Slider::new(im_str!("strength"), 0..=10).build(&ui, &mut light);
            });
        let ui_data = ui.render();

        let value = light as f32 / 10.0;
        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([value, value, value, 1.0]);
            target.draw_ui(ui_data);
        });
        tegne.end_draw();
    });
}
