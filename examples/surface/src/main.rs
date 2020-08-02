// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws a framebuffer with a custom ray-marching shader

use draw_it::ui;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        Default::default(),
        WindowOptions {
            title: "Draw-it example: Surface",
            width: 900,
            height: 900,
            ..Default::default()
        },
    )?;

    let shader = context.create_shader_from_file_watch(
        "examples/surface/shaders/raymarch.shader",
        Default::default(),
    )?;

    while window.is_open() {
        // poll events
        context.poll_events(&mut window)?;

        // draw ui
        let stats = context.stats();
        context.draw_ui(|ui| {
            ui::stats_window(&ui, stats);
        })?;

        // draw other stuff
        context.draw_on_window(|target| {
            target.set_shader(&shader);
            target.draw_surface();
        })?;
    }

    Ok(())
}
