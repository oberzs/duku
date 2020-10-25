// Oliver Berzs
// https://github.com/oberzs/draw-it

// example that draws textures

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;
use draw_it::TextureFilter;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(800, 400)
        .title("Draw-it example: Textures")
        .resizable()
        .build()?;

    let texture_1 = context.create_texture_png("examples/textures/Green/texture_01.png")?;
    let texture_2 = context.create_texture_png("examples/textures/Dark/texture_13.png")?;

    window.main_loop(move |_| {
        context.draw_on_window(None, |target| {
            target.shape_color = Color::WHITE;
            target.texture_filter = TextureFilter::Linear;
            target.draw_texture(&texture_1, (-400.0, -200.0), (400.0, 400.0));
            target.texture_filter = TextureFilter::Nearest;
            target.draw_texture(&texture_2, (0.0, -200.0), (400.0, 400.0));
        });
    });

    Ok(())
}
