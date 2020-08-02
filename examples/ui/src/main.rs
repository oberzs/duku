// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use draw_it::window::WindowOptions;
use draw_it::CameraType;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        ContextOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
        WindowOptions {
            title: "Draw-it example: UI",
            ..Default::default()
        },
    )?;

    let mut show_demo = true;

    while window.is_open() {
        context.poll_events(&mut window)?;
        context.draw_ui(|ui| {
            ui.frame.show_demo_window(&mut show_demo);
        })?;
        context.draw_on_window(|_| {})?;
    }

    Ok(())
}
