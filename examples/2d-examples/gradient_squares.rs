// Oliver Berzs
// https://github.com/oberzs/duku

use duku::Duku;
use duku::Gradient;
use duku::Result;
use duku::Rgbf;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::windowed(500, 500)?;

    let gradient = Gradient::new(vec![
        Rgbf::from("#D16BA5"),
        Rgbf::from("#86A8E7"),
        Rgbf::from("#5FFBF1"),
    ]);

    let squares = 5;
    let step = 1.0 / squares as f32;

    window.while_open(move |_| {
        duku.draw(None, |t| {
            t.translate([-200.0, 200.0, 0.0]);
            for _ in 0..squares {
                for i in 0..squares {
                    t.fill(gradient.get(i as f32 * step));
                    t.square([0.0, 0.0], 80.0);
                    t.translate_x(100.0);
                }
                t.translate([-500.0, -100.0, 0.0]);
            }
        });
    });

    Ok(())
}
