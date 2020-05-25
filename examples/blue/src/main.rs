// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// "Hello, World!" example to open a blue resizable window

use tegne::Camera;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (500, 500);

    let window = Window::new(WindowOptions {
        title: "Tegne example: Blue",
        width,
        height,
        resizable: true,
    });
    let mut tegne = Tegne::from_window(&window, Default::default());
    let mut camera = Camera::orthographic(width, height);

    window.start_loop(|events| {
        if events.is_resized() {
            let (new_width, new_height) = events.size();
            tegne.resize(new_width, new_height);
            camera.resize(new_width, new_height);
        }

        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([0.0, 0.0, 1.0, 1.0]);
        });
        tegne.end_draw();
    });
}
