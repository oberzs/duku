// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// "Hello, World!" example to open a transparent rainbow window

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .build_window(500, 500)
        .title("Draw-it example: Hello")
        .resizable()
        .build()?;

    let mut hue = 0;

    while window.is_open() {
        context.poll_events(&mut window);

        hue = (hue + 1) % 360;

        context.draw_on_window(None, |target| {
            target.clear = Color::hsv(hue, 255, 255);
        });
    }

    Ok(())
}
