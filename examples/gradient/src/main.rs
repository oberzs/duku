// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// gradient example with custom shader arguments

use draw_it::camera::CameraType;
use draw_it::color::colors;
use draw_it::error::Result;
use draw_it::ui;
use draw_it::ui::label;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;

fn main() -> Result<()> {
    let (width, height) = (800, 500);

    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Gradient",
        width,
        height,
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    )?;

    let shader = context.create_shader_from_file_watch(
        "examples/gradient/shaders/gradient.shader",
        Default::default(),
    )?;

    let material = context.create_material()?;
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
        context.draw_ui(ui)?;
        context.draw_on_window(|target| {
            target.set_shader(&shader);
            target.set_material(&material);
            target.draw_surface();
        })?;

        Ok(())
    });

    Ok(())
}
