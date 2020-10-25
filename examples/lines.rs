// Oliver Berzs
// https://github.com/oberzs/draw-it

// example that draws lines

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;
use draw_it::Target;
use draw_it::Vector2;
use std::f32::consts::PI;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(600, 400)
        .title("Draw-it example: Lines")
        .resizable()
        .build()?;

    window.main_loop(move |_| {
        context.draw_on_window(None, |target| {
            star(target, (0.0, 0.0), Color::BLUE);
            star(target, (50.0, 10.0), Color::GREEN);
            star(target, (-20.0, -40.0), Color::ORANGE);
            star(target, (20.0, 20.0), Color::SKY_BLUE);
            star(target, (-30.0, 30.0), Color::RED);
        });
    });

    Ok(())
}

fn star(target: &mut Target<'_, '_>, position: impl Into<Vector2>, color: impl Into<Color>) {
    target.push();

    target.transform.move_by(position.into().extend(0.0));
    target.line_color = color.into();

    let r = 20.0;
    for i in 0..20 {
        let q = 2.0 * PI * (i as f32 / 20.0);
        let x = r * q.cos();
        let y = r * q.sin();
        target.draw_line((0.0, 0.0, 0.0), (x, y, 0.0));
    }

    target.pop();
}
