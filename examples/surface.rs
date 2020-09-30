// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws a framebuffer with a custom ray-marching shader

use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .build_window(500, 500)
        .title("Draw-it example: Surface")
        .resizable()
        .build()?;

    // read custom shader
    let shader = context.create_shader_glsl("examples/shaders/raymarch.glsl", true)?;

    while window.is_open() {
        // poll events
        context.poll_events(&mut window);

        // draw ui
        let stats = context.stats();
        let fps = context.fps();
        let delta_time = context.delta_time();
        context.draw_ui(|ui| {
            ui.stats_window(stats, fps, delta_time);
        })?;

        // draw other stuff
        context.draw_on_window(None, |target| {
            target.shader = Some(&shader);
            target.draw_surface();
        });
    }

    Ok(())
}
