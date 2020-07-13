// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// gradient example with custom shader arguments

use tegne::colors;
use tegne::ui;
use tegne::ui::label;
use tegne::CameraType;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (800, 500);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Gradient",
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
            "examples/gradient/shaders/gradient.shader",
            Default::default(),
        )
        .unwrap();

    let material = tegne.create_material();
    let mut left_color = colors::GREEN;
    let mut right_color = colors::BLUE;

    window.main_loop(|_, ui| {
        // update material
        material.with(|m| {
            m.set_arg_1(left_color);
            m.set_arg_2(right_color);
        });

        // build ui
        ui::Window::new(label!("Background Control"))
            .size([1.0, 1.0], ui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .build(&ui, || {
                ui::color_edit(&ui, label!("Left Color"), &mut left_color);
                ui::color_edit(&ui, label!("Right Color"), &mut right_color);
            });

        // render
        tegne.draw_ui(ui);
        tegne.draw_on_window(|target| {
            target.set_shader(&shader);
            target.set_material(&material);
            target.draw_surface();
        });
    });
}
