// Oliver Berzs
// https://github.com/oberzs/duku

// Framebuffer drawing example

use duku::Camera;
use duku::Color;
use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::builder()
        .build_window(500, 500)
        .title("Duku example: Framebuffer")
        .build()?;

    let mut camera = Camera::perspective(90);
    camera.transform.move_by([-2.0, 2.0, -2.0]);
    camera.transform.look_at([0.0, 0.0, 0.0]);

    let framebuffer = duku.create_framebuffer(500, 500)?;
    let mut material = duku.create_material_pbr()?;
    material.albedo_framebuffer(&framebuffer);

    window.main_loop(move |_| {
        let fps = duku.fps();

        duku.draw(&framebuffer, None, |target| {
            // move (0, 0) to top left
            target.transform.move_left(250.0);
            target.transform.move_up(250.0);

            target.clear_color = Color::rgba_norm(0.0, 0.0, 0.0, 0.0);
            target.text_color = Color::ORANGE;

            target.draw_text(format!("Fps: {}", fps));
        });

        duku.draw_on_window(Some(&camera), |target| {
            target.draw_grid();
            target.draw_cube();

            target.set_material(&material);
            target.draw_fullscreen();
        });
    });

    Ok(())
}
