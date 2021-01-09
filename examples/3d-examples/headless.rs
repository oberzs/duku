// Oliver Berzs
// https://github.com/oberzs/duku

// This example draws a sphere on a canvas
// that is later saved out to a png.

use duku::Camera;
use duku::Duku;
use duku::Light;

fn main() {
    // create duku context without window
    let mut duku = Duku::headless();

    // create canvas to be rendered to
    let canvas = duku.create_canvas(500, 500);

    // create 3D camera with 90 fov
    let camera = Camera::perspective(90);

    // create directional light
    let light = Light::directional("#ffffff", [-1.0, -1.0, 1.0]);

    // start drawing on canvas
    duku.begin();
    duku.draw_on_canvas(&canvas, Some(&camera), |t| {
        // setup scene
        t.background("#d19c6d");
        t.light(light);
        t.translate_z(2.0);

        // draw sphere
        t.tint("#2776b5");
        t.sphere_ico([1.0, 1.0, 1.0]);
    });
    duku.end();

    // save canvas as png
    duku.save_canvas(&canvas, "canvas.png").expect("bad save");
}
