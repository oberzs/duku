// Oliver Berzs
// https://github.com/oberzs/draw-it

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

        // draw other stuff
        context.draw_on_window(None, |target| {
            target.shader = Some(&shader);
            target.draw_surface();
        });
    }

    Ok(())
}
