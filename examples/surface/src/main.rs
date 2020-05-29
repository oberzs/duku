// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example that draws a framebuffer with a custom ray-marching shader

mod ui;

use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::LevelFilter;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use simplelog::WriteLogger;
use std::fs::File;
use tegne::Camera;
use tegne::Tegne;
use tegne::Window;
use tegne::WindowOptions;

use ui::Ui;

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("example-surface.log").unwrap(),
        ),
    ])
    .unwrap();

    let (width, height) = (900, 900);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Surface",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(&mut window, Default::default());
    let camera = Camera::orthographic(width, height);

    let shader = tegne
        .create_shader_from_file_watch(
            "examples/surface/assets/raymarch.shader",
            Default::default(),
        )
        .unwrap();

    let ui = Ui::new(&tegne, width, height);

    window.main_loop(|events, _| {
        ui.draw_ui(&mut tegne, &events);

        tegne.draw_on_window(&camera, |target| {
            target.set_shader(&shader);
            target.draw_surface();
            target.blit_framebuffer(ui.framebuffer());
        });
    });
}
