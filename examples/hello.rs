// Oliver Berzs
// https://github.com/oberzs/duku

// "Hello, World!" example to open a rainbow window

use duku::Color;
use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::windowed(500, 500)?;

    let mut hue = 0;

    window.while_open(move |_| {
        hue = (hue + 1) % 360;

        duku.draw(None, |target| {
            target.clear_color = Color::hsv(hue, 255, 255);
        });
    });

    Ok(())
}
