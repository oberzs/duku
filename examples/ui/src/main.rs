// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use draw_it::camera::CameraType;
use draw_it::error::Result;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;

fn main() -> Result<()> {
    let (mut width, mut height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: UI",
        resizable: true,
        width,
        height,
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    )?;

    let mut show_demo = true;

    window.main_loop(|events, ui| {
        if let Some((new_width, new_height)) = events.resized() {
            context.resize(new_width, new_height)?;
            width = new_width;
            height = new_height;
        }

        ui.show_demo_window(&mut show_demo);

        context.draw_ui(ui)?;
        context.draw_on_window(|_| {})?;

        Ok(())
    });

    Ok(())
}
