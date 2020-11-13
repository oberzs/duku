// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws shapes

use duku::Color;
use duku::Context;
use duku::Result;
use duku::ShapeMode;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(500, 500)
        .title("Duku example: Shapes")
        .build()?;

    window.main_loop(move |_| {
        context.draw_on_window(None, |target| {
            target.clear_color = Color::gray(50);
            target.border_color = Color::BLACK;
            target.border_width = 5.0;
            target.shape_mode = ShapeMode::Center;

            target.shape_color = Color::rgb(128, 60, 220);
            target.transform.move_left(125.0);
            target.draw_square(100.0);

            target.shape_color = Color::rgb(220, 160, 60);
            target.transform.move_right(250.0);
            target.draw_circle(100.0);
        });
    });

    Ok(())
}
