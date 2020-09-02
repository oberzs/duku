// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws text

use draw_it::window::WindowOptions;
use draw_it::Color;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Quality;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        ContextOptions {
            quality: Quality::Low,
            ..Default::default()
        },
        WindowOptions {
            title: "Draw-it example: Text",
            width: 600,
            height: 400,
            resizable: true,
        },
    )?;

    let mut material_1 = context.create_material();
    material_1.set_font_color(Color::RED);
    material_1.update();
    let mut material_2 = context.create_material();
    material_2.set_font_color(Color::BLUE);
    material_2.update();

    let left = -290.0;

    while window.is_open() {
        context.poll_events(&mut window);

        context.draw_on_window(None, |target| {
            target.clear = Color::BLACK;
            target.text.draw("Bitmap 24p text", [left, 190.0, 1.0]);

            // render with custom material
            target.text.color = Color::RED;
            target.text.draw("Red text!", [left, 160.0, 1.0]);
            target.text.color = Color::BLUE;
            target
                .text
                .draw("Blue text\n.. on multiple lines", [left, 130.0, 1.0]);
        });
    }

    Ok(())
}
