// Oliver Berzs
// https://github.com/oberzs/duku

// "Hello, World!" example to open a transparent rainbow window

use duku::Color;
use duku::Context;
use duku::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(500, 500)
        .title("Duku example: Hello")
        .resizable()
        .build()?;

    let mut hue = 0;

    window.main_loop(move |_| {
        hue = (hue + 1) % 360;

        context.draw_on_window(None, |target| {
            target.clear_color = Color::hsv(hue, 255, 255);
        });
    });

    Ok(())
}
