// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws lines

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;
use draw_it::Target;
use draw_it::Vector2;
use std::f32::consts::PI;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .build_window(600, 400)
        .title("Draw-it example: Lines")
        .resizable()
        .build()?;

    while window.is_open() {
        context.poll_events(&mut window);

        context.draw_on_window(None, |target| {
            star(target, (0.0, 0.0), Color::BLUE);
            star(target, (50.0, 10.0), Color::GREEN);
            star(target, (-20.0, -40.0), Color::ORANGE);
            star(target, (20.0, 20.0), Color::SKY_BLUE);
            star(target, (-30.0, 30.0), Color::RED);
        });
    }

    Ok(())
}

fn star<V, C>(target: &mut Target<'_, '_>, position: V, color: C)
where
    V: Into<Vector2>,
    C: Into<Color>,
{
    let pos = position.into();

    target.line_color = color.into();

    let r = 20.0;
    for i in 0..20 {
        let q = 2.0 * PI * (i as f32 / 20.0);
        let x = pos.x + r * q.cos();
        let y = pos.y + r * q.sin();
        target.draw_line((pos.x, pos.y, 0.0), (x, y, 0.0));
    }
}
