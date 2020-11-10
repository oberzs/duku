// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws textures

use duku::Color;
use duku::ColorSpace;
use duku::Context;
use duku::Filter;
use duku::Mips;
use duku::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(800, 400)
        .title("Duku example: Textures")
        .resizable()
        .build()?;

    let texture_1 = context.create_texture_png(
        "examples/textures/prototype/green.png",
        ColorSpace::Srgb,
        Mips::Log2,
    )?;

    let colors: Vec<_> = (0..64).map(|_| Color::WHITE).collect();
    let texture_2 = context.create_texture(&colors, ColorSpace::Srgb, 8, 8, Mips::Zero);

    let mut hue = 0;

    window.main_loop(move |_| {
        hue = (hue + 1) % 360;

        let tex = context.texture_mut(&texture_2);
        tex.set_pixel(1, 1, Color::hsv(hue, 255, 255));
        tex.set_pixel(6, 1, Color::hsv(hue, 255, 255));
        tex.set_pixel(1, 6, Color::hsv(hue, 255, 255));
        tex.set_pixel(6, 6, Color::hsv(hue, 255, 255));

        context.draw_on_window(None, |target| {
            target.draw_texture(&texture_1, [-400.0, -200.0], [400.0, 400.0]);
            target.texture_filter = Filter::Nearest;
            target.draw_texture(&texture_2, [0.0, -200.0], [400.0, 400.0]);
        });
    });

    Ok(())
}
