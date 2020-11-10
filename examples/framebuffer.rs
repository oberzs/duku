// Oliver Berzs
// https://github.com/oberzs/duku

// Framebuffer drawing example

use duku::Camera;
use duku::Color;
use duku::Context;
use duku::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(500, 500)
        .title("Duku example: Framebuffer")
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

            target.set_material(&material);
            target.draw_fullscreen();
        });
    });

    Ok(())
}
