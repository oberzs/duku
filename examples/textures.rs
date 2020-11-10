// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws textures

use duku::Color;
use duku::Context;
use duku::Mips;
use duku::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(800, 400)
        .title("Duku example: Textures")
        .resizable()
        .build()?;

    let texture_1 =
        context.create_texture_png("examples/textures/prototype/green.png", Mips::Log2)?;
    let texture_2 =
        context.create_texture_png("examples/textures/prototype/dark.png", Mips::Zero)?;

    let mut hue = 0;

    window.main_loop(move |_| {
        hue = (hue + 1) % 360;

        let tex = context.texture_mut(&texture_2);
        tex.set_pixel(10, 10, Color::hsv(hue, 255, 255));
        tex.set_pixel(11, 10, Color::hsv(hue, 255, 255));
        tex.set_pixel(10, 11, Color::hsv(hue, 255, 255));
        tex.set_pixel(11, 11, Color::hsv(hue, 255, 255));

        context.draw_on_window(None, |target| {
            target.draw_texture(&texture_1, [-400.0, -200.0], [400.0, 400.0]);
            target.draw_texture(&texture_2, [0.0, -200.0], [400.0, 400.0]);
        });
    });

    Ok(())
}
