// Oliver Berzs
// https://github.com/oberzs/duku

// This example loads and draws glTF ship model

use duku::window::Orbit;
use duku::Camera;
use duku::Duku;
use duku::Light;
use duku::Result;

fn main() -> Result<()> {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500)?;

    // create 3D camera with 90 fov and move it
    let mut camera = Camera::perspective(90);
    camera.move_by([8.0, 8.0, -8.0]);

    // create basic orbit mode camera controller
    let mut orbit = Orbit::new([0.0, 0.0, 0.0]);

    // load glTF ship model and fix it's color space
    // cause it was exported incorrectly
    let ship = duku.create_model_gltf("examples/models/ship_dark.gltf", None)?;
    ship.write().fix_color_space();

    // create directional light
    let mut light = Light::directional("#FFFAEB", [-0.7, -0.5, 1.0]);
    light.brightness = 2.0;

    // start window loop
    window.while_open(move |events| {
        // update camera based on controller
        orbit.update(&mut camera, events, duku.delta_time());

        // start drawing on window
        duku.draw(Some(&camera), |t| {
            // setup scene
            t.light(light);
            t.ambient("#E6EDFF", 0.03);
            t.debug_grid();

            // transform scene to
            // make ship in the center
            t.rotate_y(180.0);
            t.translate_x(4.0);

            // draw ship model
            t.model(&ship);
        });
    });

    Ok(())
}
