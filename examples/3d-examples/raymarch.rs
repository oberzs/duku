// Oliver Berzs
// https://github.com/oberzs/duku

// This example loads a custom shader from a glsl file
// and draws with it on the window.
// Also records a GIF of the window.

use duku::gif::Gif;
use duku::window::Key;
use duku::Duku;
use duku::Result;

#[derive(PartialEq)]
enum RecordingStatus {
    Before,
    During,
    After,
}

fn main() -> Result<()> {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500);

    // load custom glsl shader from file
    let shader = duku.create_shader_glsl("examples/shaders/raymarch.glsl")?;

    // create GIF file for encoding
    let mut gif = Gif::default();
    let mut status = RecordingStatus::Before;

    // start window loop
    window.while_open(move |events| {
        // start drawing on window
        duku.begin();
        duku.draw(None, |t| {
            // draw a surface with the shader
            t.surface(&shader);
        });
        duku.end();

        // handle GIF recording
        if events.is_key_pressed(Key::Space) {
            if status == RecordingStatus::Before {
                status = RecordingStatus::During;
            }
            if status == RecordingStatus::During {
                duku.encode_window_canvas(&mut gif).expect("bad encode");
            }
        } else if events.is_key_released(Key::Space) && status == RecordingStatus::During {
            status = RecordingStatus::After;
            println!("* Exporting gif");
            gif.save("raymarch.gif").expect("bad save");
        }
    });

    Ok(())
}
