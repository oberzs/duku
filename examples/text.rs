// Oliver Berzs
// https://github.com/oberzs/draw-it

// example that draws text

use draw_it::window::Key;
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

    let mut input = String::new();

    window.main_loop(move |events| {
        // update text
        if let Some(c) = events.typed_char() {
            input.push(c);
        }
        if events.is_key_typed(Key::Back) {
            input.pop();
        }

        context.draw_on_window(None, |target| {
            // move (0, 0) to top left
            target.transform.move_left(300.0);
            target.transform.move_up(200.0);

            target.clear_color = Color::BLACK;

            // left and top margin
            target.transform.move_right(10.0);
            target.transform.move_down(10.0);

            target.text_color = Color::WHITE;
            target.draw_text("Bitmap 24p text", [0.0, 0.0]);
            target.text_color = Color::RED;
            target.transform.move_down(40.0);
            target.draw_text("Red text!", [0.0, 0.0]);
            target.text_color = Color::BLUE;
            target.transform.move_down(40.0);
            target.draw_text(
                "Blue text\n.. on multiple lines.\nTry writing some text",
                [0.0, 0.0],
            );
            target.text_color = Color::ORANGE;
            target.transform.move_down(80.0);
            target.draw_text(&input, [0.0, 0.0]);
        });
    });

    Ok(())
}
