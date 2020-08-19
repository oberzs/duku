// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws a framebuffer with a custom ray-marching shader

use draw_it::window::WindowOptions;
use draw_it::Camera;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (width, height) = (900, 900);
    let (mut context, mut window) = Context::with_window(
        Default::default(),
        WindowOptions {
            title: "Draw-it example: Surface",
            width,
            height,
            ..Default::default()
        },
    )?;

    // read custom shader
    // in debug mode, read from file with hot-reload
    // in release mode, embed in executable
    #[cfg(debug_assertions)]
    let shader =
        context.create_shader_from_file_watch("examples/surface/shaders/raymarch.shader")?;
    #[cfg(not(debug_assertions))]
    let shader = context.create_shader(include_bytes!("../shaders/raymarch.shader"))?;

    let camera = Camera::orthographic(width as f32, height as f32);

    while window.is_open() {
        // poll events
        context.poll_events(&mut window)?;

        // draw ui
        let stats = context.stats();
        let fps = context.fps();
        let delta_time = context.delta_time();
        context.draw_ui(|ui| {
            ui.stats_window(stats, fps, delta_time);
        })?;

        // draw other stuff
        context.draw_on_window(&camera, |target| {
            target.set_shader(&shader);
            target.draw_surface();
        })?;
    }

    Ok(())
}
