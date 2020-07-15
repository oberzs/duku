// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example that draws a framebuffer with a custom ray-marching shader

use tegne::ui;
use tegne::window::Window;
use tegne::window::WindowOptions;
use tegne::Context;

fn main() {
    let (width, height) = (900, 900);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Surface",
        width,
        height,
        ..Default::default()
    });
    let mut context = Context::from_window(&mut window, Default::default());

    let shader = context
        .create_shader_from_file_watch(
            "examples/surface/shaders/raymarch.shader",
            Default::default(),
        )
        .unwrap();

    window.main_loop(|events, ui| {
        ui::stats_window(&ui, &context, events);

        context.draw_ui(ui);
        context.draw_on_window(|target| {
            target.set_shader(&shader);
            target.draw_surface();
        });
    });
}
