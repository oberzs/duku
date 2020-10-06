// Oliver Berzs
// https://github.com/oberzs/draw-it

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .build_window(500, 500)
        .title("Draw-it example: UI")
        .resizable()
        .build()?;

    let mut show_demo = true;

    while window.is_open() {
        context.poll_events(&mut window);
        context.draw_ui(|ui| {
            ui.frame.show_demo_window(&mut show_demo);
        })?;
        context.draw_on_window(None, |_| {});
    }

    Ok(())
}
