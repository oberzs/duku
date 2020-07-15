// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI example using Imgui support
// https://github.com/Gekkio/imgui-rs

use tegne::camera::CameraType;
use tegne::window::Window;
use tegne::window::WindowOptions;
use tegne::Context;
use tegne::ContextOptions;

fn main() {
    let (mut width, mut height) = (500, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: UI",
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
    );

    let mut show_demo = true;

    window.main_loop(|events, ui| {
        if let Some((new_width, new_height)) = events.resized() {
            context.resize(new_width, new_height);
            width = new_width;
            height = new_height;
        }

        ui.show_demo_window(&mut show_demo);

        context.draw_ui(ui);
        context.draw_on_window(|_| {});
    });
}
