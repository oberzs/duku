// Oliver Berzs
// https://github.com/oberzs/draw-it

// example that draws lines

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;
use draw_it::ShapeMode;
use draw_it::Vector2;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(600, 400)
        .title("Draw-it example: Shapes")
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
            target.draw_rectangle([10.0, 10.0], [150.0, 100.0]);

            target.shape_color = Color::RED;
            target.draw_square([250.0, 50.0], 50.0);

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
