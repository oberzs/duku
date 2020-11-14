// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws textures

use duku::ColorSpace;
use duku::Duku;
use duku::Mips;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::builder()
        .build_window(800, 400)
        .title("Duku example: Textures")
        .build()?;

    let texture_1 = duku.create_texture_png(
        "examples/textures/prototype/green.png",
        ColorSpace::Srgb,
        Mips::Log2,
    )?;

    let texture_2 =
        duku.create_texture_jpeg("examples/textures/cat.jpg", ColorSpace::Srgb, Mips::Log2)?;

    window.main_loop(move |_| {
        duku.draw_on_window(None, |target| {
            target.draw_texture(&texture_1, [-400.0, -200.0], [400.0, 400.0]);
            target.draw_texture(&texture_2, [0.0, -200.0], [400.0, 400.0]);
        });
    });

    Ok(())
}
