// Oliver Berzs
// https://github.com/oberzs/duku

use duku::window::Orbit;
use duku::Camera;
use duku::Duku;
use duku::Light;
use duku::Result;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::windowed(500, 500)?;

    let mut camera = Camera::perspective(90);
    camera.move_back(2.0);
    let mut orbit = Orbit::new([0.0, 0.0, 0.0]);

    let cube = duku.create_model_gltf("examples/models/cube.glb", None)?;
    // ship.fix_color_space();

    let light = Light::directional("#ffffff", [-0.7, -0.5, 1.0]);

    window.while_open(move |events| {
        orbit.update(&mut camera, events, duku.delta_time());

        duku.draw(Some(&camera), |t| {
            t.light(light);
            t.debug_grid();

            t.model(&cube);

            t.translate_x(3.0);
            t.tint("#00ffff");
            t.cube([2.0, 2.0, 2.0]);
        });
    });

    Ok(())
}
