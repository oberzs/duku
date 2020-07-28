// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws a framebuffer with a custom ray-marching shader

use draw_it::error::Result;
use draw_it::ui;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;

fn main() -> Result<()> {
    let (width, height) = (900, 900);

    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Surface",
        width,
        height,
        ..Default::default()
    });
    let mut context = Context::from_window(&mut window, Default::default())?;

    let shader = context.create_shader_from_file_watch(
        "examples/surface/shaders/raymarch.shader",
        Default::default(),
    )?;

    window.main_loop(|events, ui| {
        ui::stats_window(&ui, &context, events);

        context.draw_ui(ui)?;
        context.draw_on_window(|target| {
            target.set_shader(&shader);
            target.draw_surface();
        })?;

        Ok(())
    });

    Ok(())
}
