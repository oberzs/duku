// Oliver Berzs
// https://github.com/oberzs/draw-it

// example that draws text

use draw_it::Color;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .low_quality()
        .build_window(600, 400)
        .title("Draw-it example: Text")
        .resizable()
        .build()?;

    let left = -290.0;

    window.main_loop(move |events| {
        context.handle_window_events(events);

        context.draw_on_window(None, |target| {
            target.clear_color = Color::BLACK;
            target.text_color = Color::WHITE;
            target.draw_text("Bitmap 24p text", (left, 190.0));
            target.text_color = Color::RED;
            target.draw_text("Red text!", (left, 160.0));
            target.text_color = Color::BLUE;
            target.draw_text("Blue text\n.. on multiple lines", (left, 130.0));
        });
    });

    Ok(())
}
