// Oliver Berzs
// https://github.com/oberzs/duku

// This example draws a openable and closable textbox
// using shapes and text

use duku::otf::CharSet;
use duku::window::Cursor;
use duku::window::Events;
use duku::window::MouseButton;
use duku::Duku;
use duku::Result;
use duku::Rgb;
use duku::ShapeMode;
use duku::Target;

fn main() -> Result<()> {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500);

    // load font-awesome font for 'x' icon
    let fontawesome = duku.create_font_otf(
        "examples/fonts/fontawesome.ttf",
        24,
        Some(CharSet::Custom("\u{f00d}\u{f27a}")),
    )?;

    let mut is_textbox_open = false;

    // start window loop
    window.while_open(move |events| {
        // depending on if textbox is open or not
        // check if mouse is clicked on the button
        if is_textbox_open {
            let on_button = is_hovering(events, 375.0, 175.0, 50.0, 50.0);
            if events.is_button_clicked(MouseButton::Left) && on_button {
                is_textbox_open = false;
            }
        } else {
            let on_button = is_hovering(events, 250.0, 250.0, 100.0, 50.0);
            if events.is_button_clicked(MouseButton::Left) && on_button {
                is_textbox_open = true;
            }
        }

        // start drawing on window
        duku.begin();
        duku.draw(None, |t| {
            if is_textbox_open {
                // translate scene forwards to
                // allow room for UI in front
                t.translate_z(5.0);

                // draw textbox
                textbox(t, 0.0, 0.0, 300.0, 200.0);

                // translate scene back to
                // draw the rest in front
                t.translate_z(-1.0);

                // set shape drawing mode to draw from top-left
                t.shape_mode(ShapeMode::Center);

                // draw textbox text
                t.text(
                    "This is a textbox\nthat you can close\nwith the 'x' button on\nthe top right!",
                    [0.0, 0.0],
                );

                // translate scene back to
                // draw the rest in front
                t.translate_z(-1.0);

                // set shape drawing mode to draw from center
                t.shape_mode(ShapeMode::Center);

                // draw background for 'x' button
                textbox(t, 125.0, 75.0, 50.0, 50.0);

                // translate scene back to
                // draw the rest in front
                t.translate_z(-1.0);

                // set font to fontawesome and draw 'x' icon
                t.font(&fontawesome);
                t.text("\u{f00d}", [125.0, 75.0]);
            } else {
                // translate scene forwards to
                // allow room for UI in front
                t.translate_z(5.0);

                // draw textbox
                textbox(t, 0.0, 0.0, 50.0, 50.0);

                // translate scene back to
                // draw text in front
                t.translate_z(-1.0);

                // draw 'Open' label
                t.font(&fontawesome);
                t.text("\u{f27a}", [0.0, 0.0]);
            }
        });
        duku.end();
    });

    Ok(())
}

fn is_hovering(events: &mut Events, x: f32, y: f32, width: f32, height: f32) -> bool {
    // check if position is inside area
    let mouse_pos = events.mouse_position();
    let hovering = mouse_pos.x < x + width / 2.0
        && mouse_pos.x > x - width / 2.0
        && mouse_pos.y < y + height / 2.0
        && mouse_pos.y > y - height / 2.0;

    // set cursor to hand if inside area
    if hovering {
        events.set_cursor(Cursor::Hand);
    } else {
        events.set_cursor(Cursor::Arrow);
    }

    hovering
}

fn textbox(t: &mut Target, x: f32, y: f32, width: f32, height: f32) {
    // draw outer textbox rectangle
    t.fill("#000000");
    t.stroke(Rgb::clear());
    t.rect([x, y], [width, height]);

    // translate scene back to
    // draw the rest in front
    t.translate_z(-1.0);

    // draw inner textbox rectangle
    t.stroke("ffffff");
    t.fill(Rgb::clear());
    t.stroke_weight(5.0);
    t.rect([x, y], [width - 10.0, height - 10.0]);
}
