// Oliver Berzs
// https://github.com/oberzs/duku

// This example draws a cube in the center of the window,
// rotating and coloring it based on the time that has passed.

use duku::Camera;
use duku::Duku;
use duku::Hsb;
use duku::Light;
use duku::Result;
use std::time::Instant;

fn main() -> Result<()> {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500)?;

    // create 3D camera with 90 fov
    let camera = Camera::perspective(90);

    // create main directional light
    // that can cast shadows
    let light = Light::main([-1.0, -1.0, 1.0], "#ffffff", 1.0);

    // start timer for rotation and color
    let timer = Instant::now();

    // start window loop
    window.while_open(move |_| {
        // start drawing on window
        duku.draw(Some(&camera), |t| {
            // setup scene
            t.background("#ababab");
            t.light(light);
            t.shadows();

            // get elapsed time since start
            let elapsed = timer.elapsed().as_secs_f32();

            // transform scene
            let angle = elapsed * 45.0;
            t.rotate_x(angle);
            t.rotate_y(angle);
            t.translate([0.0, 0.0, 2.0]);

            // draw cube
            let hue = (elapsed * 60.0) as u16;
            t.tint(Hsb::new(hue, 70, 80));
            t.cube([1.0, 1.0, 1.0]);
        });
    });

    Ok(())
}
