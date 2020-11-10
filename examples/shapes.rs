// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws lines

use duku::Color;
use duku::Context;
use duku::Result;
use duku::ShapeMode;
use duku::Vector2;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(600, 400)
        .title("Duku example: Shapes")
        .resizable()
        .build()?;

    window.main_loop(move |_| {
        context.draw_on_window(None, |target| {
            // move (0, 0) to top left
            target.transform.move_left(300.0);
            target.transform.move_up(200.0);
            target.transform.scale.y = -1.0;

            target.border_width = 5.0;

            target.shape_color = Color::GREEN;
            target.draw_rectangle([150.0, 100.0]);

            target.shape_color = Color::RED;
            target.draw_square(50.0);

            target.shape_mode = ShapeMode::Center;

            target.push();
            target.shape_color = Color::TEAL;
            target.transform.move_forward(1.0);
            target.draw_ellipse([300.0, 200.0], [100.0, 50.0]);
            target.pop();

            target.shape_color = Color::AQUA;
            target.draw_circle([300.0, 200.0], 25.0);

            target.shape_color = Color::BLUE;
            target.draw_shape(&[
                Vector2::new(400.0, 300.0),
                Vector2::new(450.0, 300.0),
                Vector2::new(475.0, 350.0),
                Vector2::new(425.0, 375.0),
                Vector2::new(350.0, 350.0),
            ]);
        });
    });

    Ok(())
}
