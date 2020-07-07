// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example to render variable thickness lines

use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (mut width, mut height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Lines",
        resizable: true,
        width,
        height,
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());

    window.main_loop(|events, _| {
        if let Some((new_width, new_height)) = events.resized() {
            tegne.resize(new_width, new_height);
            width = new_width;
            height = new_height;
        }

        tegne.draw_on_window(|_| {});
    });
}
