// Oliver Berzs
// https://github.com/oberzs/duku

// This example loads a custom shader from a glsl file
// and draws with it on the window.

use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500)?;

    // load custom glsl shader from file
    let shader = duku.create_shader_glsl("examples/shaders/raymarch.glsl")?;

    // start window loop
    window.while_open(move |_| {
        // start drawing on window
        duku.draw(None, |t| {
            // draw a surface with the shader
            t.surface(&shader);
        });
    });

    Ok(())
}
