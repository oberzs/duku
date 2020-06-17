// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// "Hello, World!" example to open a blue resizable window
// blue is actually rainbow

use tegne::begin_profile;
use tegne::end_profile;
use tegne::Color;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    begin_profile("Startup", "startup.json");
    let (mut width, mut height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Blue",
        resizable: true,
        width,
        height,
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());

    let mut hue = 0;
    end_profile();

    window.main_loop(|events, _| {
        if let Some((new_width, new_height)) = events.resized() {
            tegne.resize(new_width, new_height);
            width = new_width;
            height = new_height;
        }

        hue = (hue + 1) % 360;

        tegne.draw_on_window(|target| {
            target.set_clear(Color::hsv(hue, 255, 255));
        });
    });
}
