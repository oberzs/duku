// Oliver Berzs
// https://github.com/oberzs/draw-it

// "Hello, World!" example to open a transparent rainbow window

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(500, 500)
        .title("Draw-it example: Hello")
        .resizable()
        .build()?;

    let mut hue = 0;

    window.main_loop(move |events| {
        context.handle_window_events(events);

        hue = (hue + 1) % 360;

        context.draw_on_window(None, |target| {
            target.clear_color = Color::hsv(hue, 255, 255);
        });
    });

    Ok(())
}
