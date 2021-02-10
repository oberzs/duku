// Oliver Berzs
// https://github.com/oberzs/duku

// This example creates a canvas that is not cleared
// which allows painting on it.

use duku::window::MouseButton;
use duku::Clear;
use duku::Duku;

fn main() {
    // create duku context and window
    // without MSAA to allow not clearing the background
    let (mut duku, window) = Duku::builder().no_msaa().build_window(500, 500).build();

    // create canvas to be painted on
    let canvas = duku.create_canvas(500, 500);

    // start window loop
    window.while_open(move |events| {
        // start drawing on window
        duku.begin();
        duku.draw_on_canvas(&canvas, None, |t| {
            t.clear(Clear::Depth);
            t.translate([-250.0, 250.0, 0.0]);
            if events.is_button_pressed(MouseButton::Left) {
                t.fill("#aaffaa");
                t.stroke("#aaffaa");

                let mouse = events.mouse_position();
                t.circle([mouse.x, -mouse.y], 50.0);
            }
        });
        duku.draw(None, |t| {
            t.fullscreen(&canvas);
        });
        duku.end();
    });
}
