// Oliver Berzs
// https://github.com/oberzs/duku

// example that draws a framebuffer with a custom ray-marching shader

use duku::Context;
use duku::Result;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .build_window(500, 500)
        .title("Duku example: Surface")
        .resizable()
        .build()?;

    // read custom shader
    let shader = context.create_shader_glsl("examples/shaders/raymarch.glsl", true)?;

    window.main_loop(move |_| {
        context.draw_on_window(None, |target| {
            target.set_shader(&shader);
            target.draw_surface();
        });
    });

    Ok(())
}
