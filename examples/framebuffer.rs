// Oliver Berzs
// https://github.com/oberzs/draw-it

// Framebuffer drawing example

use draw_it::Camera;
use draw_it::Color;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(500, 500)
        .title("Draw-it example: Framebuffer")
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by([-2.0, 2.0, -2.0]);
    camera.transform.look_at([0.0, 0.0, 0.0]);

    let framebuffer = context.create_framebuffer(500, 500);
    let material = context
        .build_material()
        .albedo_framebuffer(&framebuffer)
        .build();

    window.main_loop(move |_| {
        let fps = context.fps();

        context.draw(&framebuffer, None, |target| {
            target.clear_color = Color::rgba_norm(0.0, 0.0, 0.0, 0.0);
            target.text_color = Color::ORANGE;
            target.draw_text(format!("Fps: {}", fps), [-250.0, 250.0]);
        });

        context.draw_on_window(Some(&camera), |target| {
            target.draw_grid();
            target.draw_cube();

            target.material = Some(&material);
            target.draw_fullscreen();
        });
    });

    Ok(())
}
