// Oliver Berzs
// https://github.com/oberzs/duku

// Canvas drawing example

use duku::Camera;
use duku::Color;
use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::windowed(500, 500)?;

    let mut camera = Camera::perspective(90);
    camera.transform.move_by([-2.0, 2.0, -2.0]);
    camera.transform.look_at([0.0, 0.0, 0.0]);

    let canvas = duku.create_canvas(500, 500)?;

    window.while_open(move |_| {
        let fps = duku.fps();

        duku.draw_on_canvas(&canvas, None, |target| {
            // move (0, 0) to top left
            target.transform.move_left(250.0);
            target.transform.move_up(250.0);

            target.clear_color = Color::rgba_norm(0.0, 0.0, 0.0, 0.0);
            target.text_color = Color::ORANGE;

            target.draw_text(format!("Fps: {}", fps));
        });

        duku.draw(Some(&camera), |target| {
            target.draw_grid();
            target.draw_cube();

            target.draw_fullscreen(&canvas);
        });
    });

    Ok(())
}
