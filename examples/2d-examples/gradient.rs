// Oliver Berzs
// https://github.com/oberzs/duku

// This example creates a gradient and draws a circle with
// circles of gradient colors

use duku::Duku;
use duku::Gradient;
use duku::Rgbf;
use std::f32::consts::PI;

fn main() {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500);

    // create gradient with 4 colors where the start
    // and end colors are the same
    let gradient = Gradient::new(vec![
        Rgbf::from("#D16BA5"),
        Rgbf::from("#86A8E7"),
        Rgbf::from("#5FFBF1"),
        Rgbf::from("#D16BA5"),
    ]);

    let big_r = 150.0;
    let small_r = 35.0;
    let count = 20;

    // start window loop
    window.while_open(move |_| {
        // start drawing on window
        duku.begin();
        duku.draw(None, |t| {
            // for each circle
            for i in 0..count {
                // calculate the percentage of how
                // far around the circle we are
                let perc = i as f32 / count as f32;

                // calculate the angle or the point
                let angle = perc * 2.0 * PI;

                // calculate the position
                // based on polar coordinates
                let x = big_r * angle.cos();
                let y = big_r * angle.sin();

                // set the fill to a color from the
                // gradient
                t.fill(gradient.get(perc));

                // draw a circle with that color
                t.circle([x, y], small_r);
            }
        });
        duku.end();
    });
}
