// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws text

use duku::window::Key;
use duku::BorderMode;
use duku::Color;
use duku::Duku;
use duku::Result;
use duku::ShapeMode;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::builder()
        .no_msaa()
        .build_window(500, 500)
        .title("Duku example: Text")
        .resizable()
        .build()?;

    let mut input = String::new();
    let mut time = 0.0f32;

    window.main_loop(move |events| {
        // update text
        if let Some(c) = events.typed_char() {
            input.push(c);
        }
        if events.is_key_typed(Key::Back) {
            input.pop();
        }

        let font = duku.font(&duku.builtins.fira_font);
        let input_length = font.text_width(&input);

        duku.draw_on_window(None, |target| {
            // move (0, 0) to top left
            target.transform.move_left(250.0);
            target.transform.move_up(250.0);

            target.clear_color = Color::BLACK;

            // left and top margin
            target.transform.move_right(10.0);
            target.transform.move_down(10.0);

            target.text_color = Color::WHITE;
            target.draw_text("Bitmap 24p text");
            target.text_color = Color::RED;
            target.new_line();
            target.draw_text("Red text!");
            target.text_color = Color::BLUE;
            target.new_line();
            target.draw_text("Blue text\n.. on multiple lines.\nTry writing some text");
            target.text_color = Color::ORANGE;
            target.new_line();
            target.new_line();
            target.new_line();
            target.draw_text(&input);

            // draw cursor at the end of input
            target.shape_color = Color::rgba_norm(1.0, 0.5, 0.0, time.sin() * 0.5 + 0.5);
            target.shape_mode = ShapeMode::Center;
            target.border_mode = BorderMode::Disabled;
            target.transform.move_right(input_length as f32 + 5.0);
            target.transform.move_down(12.0);
            target.draw_rectangle([5.0, 20.0]);
        });

        time += 0.1;
    });

    Ok(())
}
