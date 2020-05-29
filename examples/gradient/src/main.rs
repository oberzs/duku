// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// gradient example with custom shader arguments

use tegne::colors;
use tegne::CameraType;
use tegne::MaterialOptions;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (800, 500);
    let color_1 = colors::RED;
    let color_2 = colors::BLUE;

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Surface",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(
        &mut window,
        TegneOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    );

    let shader = tegne
        .create_shader_from_file_watch(
            "examples/gradient/assets/gradient.shader",
            Default::default(),
        )
        .unwrap();
    let material = tegne.create_material(MaterialOptions {
        arg_1: color_1.to_rgba_norm_vec(),
        arg_2: color_2.to_rgba_norm_vec(),
        ..Default::default()
    });

    window.main_loop(|_, _| {
        tegne.draw_on_window(|target| {
            target.set_shader(&shader);
            target.set_material(&material);
            target.draw_surface();
        });
    });
}
