// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// "Hello, World!" example to open a transparent rainbow window

use draw_it::window::WindowOptions;
use draw_it::Color;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        Default::default(),
        WindowOptions {
            title: "Draw-it example: Hello",
            resizable: true,
            transparent: true,
            ..Default::default()
        },
    )?;

    let mut hue = 0;

    while window.is_open() {
        context.poll_events(&mut window);

        hue = (hue + 1) % 360;
        let mut color = Color::hsv(hue, 255, 255);
        color.a = 50;

        context.draw_on_window(None, |target| {
            target.clear = color;
        });
    }

    Ok(())
}
