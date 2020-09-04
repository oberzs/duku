// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws text

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .low_quality()
        .build_window(600, 400)
        .title("Draw-it example: Text")
        .resizable()
        .build()?;

    let left = -290.0;

    while window.is_open() {
        context.poll_events(&mut window);

        context.draw_on_window(None, |target| {
            target.clear = Color::BLACK;
            target.text.draw("Bitmap 24p text", (left, 190.0));

            // render with custom material
            target.text.color = Color::RED;
            target.text.draw("Red text!", (left, 160.0));
            target.text.color = Color::BLUE;
            target
                .text
                .draw("Blue text\n.. on multiple lines", (left, 130.0));
        });
    }

    Ok(())
}
