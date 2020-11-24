// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws a gltf model

use duku::window::Controller;
use duku::Camera;
use duku::Color;
use duku::Duku;
use duku::Light;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::builder()
        .build_window(500, 500)
        .title("Duku example: Gltf")
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by([0.0, 0.5, -2.0]);
    camera.transform.look_at([0.0, 0.0, 0.0]);

    let mut controller = Controller::orbit([0.0, 0.0, 0.0]);

    let helmet = duku.create_model_gltf("examples/models/helmet/DamagedHelmet.gltf")?;

    window.main_loop(move |events| {
        controller.update(&mut camera, events, duku.delta_time());

        duku.draw_on_window(Some(&camera), |target| {
            target.clear_color = Color::gray(50);
            target.lights[0] = Light::main([-1.0, -2.0, 1.0], Color::rgb(255, 250, 235), 5.0);
            target.shadow_depth = 5.0;

            target.draw_grid();
            target.draw_model(&helmet);
        });
    });

    Ok(())
}
