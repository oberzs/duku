// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example that draws a framebuffer with a custom ray-marching shader

use tegne::ui;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (900, 900);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Surface",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());

    let shader = tegne
        .create_shader_from_file_watch(
            "examples/surface/shaders/raymarch.shader",
            Default::default(),
        )
        .unwrap();

    window.main_loop(|events, ui| {
        ui::stats_window(&ui, &tegne, events);

        tegne.draw_ui(ui);
        tegne.draw_on_window(|target| {
            target.set_shader(&shader);
            target.draw_surface();
        });
    });
}
