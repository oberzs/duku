// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws lines

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;
use draw_it::Vector2;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .build_window(600, 400)
        .title("Draw-it example: Shapes")
        .resizable()
        .build()?;

    while window.is_open() {
        context.poll_events(&mut window);

        context.draw_on_window(None, |target| {
            target.shape_color = Color::GREEN;
            target.draw_rectangle((0.0, 0.0), (100.0, 100.0));

            target.shape_color = Color::BLUE;
            target.draw_shape(&[
                Vector2::new(-100.0, 0.0),
                Vector2::new(-50.0, 100.0),
                Vector2::new(-25.0, 50.0),
                Vector2::new(-50.0, 0.0),
                Vector2::new(-100.0, -50.0),
            ]);
        });
    }

    Ok(())
}
