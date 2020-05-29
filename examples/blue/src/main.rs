// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// "Hello, World!" example to open a blue resizable window

use tegne::colors;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Blue",
        resizable: true,
        width,
        height,
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());

    window.main_loop(|events, _| {
        if events.is_resized() {
            let (new_width, new_height) = events.size();
            tegne.resize(new_width, new_height);
        }

        tegne.draw_on_window(|target| {
            target.set_clear(colors::BLUE);
        });
    });
}
