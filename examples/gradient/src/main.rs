// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// gradient example with custom shader arguments

use tegne::ui;
use tegne::ui::im_str;
use tegne::CameraType;
use tegne::Color;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (800, 500);

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
    let material = tegne.create_material(Default::default());

    let mut color_1 = [0.0, 0.0, 0.0];
    let mut color_2 = [0.0, 0.0, 0.0];

    window.main_loop(|_, ui| {
        // update material
        tegne.with_material(&material, |m| {
            m.set_arg_1(Color::from(color_1));
            m.set_arg_2(Color::from(color_2));
        });

        // build ui
        ui::Window::new(im_str!("Left control"))
            .size([300.0, 300.0], ui::Condition::FirstUseEver)
            .position([5.0, 5.0], ui::Condition::FirstUseEver)
            .build(&ui, || {
                ui::ColorPicker::new(im_str!("color"), &mut color_1).build(&ui);
            });
        ui::Window::new(im_str!("Right control"))
            .size([300.0, 300.0], ui::Condition::FirstUseEver)
            .position([310.0, 5.0], ui::Condition::FirstUseEver)
            .build(&ui, || {
                ui::ColorPicker::new(im_str!("color"), &mut color_2).build(&ui);
            });
        let ui_data = ui.render();

        // render
        tegne.draw_on_window(|target| {
            target.set_shader(&shader);
            target.set_material(&material);
            target.draw_surface();
            target.draw_ui(ui_data);
        });
    });
}
