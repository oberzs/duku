// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// gradient example with custom shader arguments

use draw_it::window::WindowOptions;
use draw_it::CameraType;
use draw_it::Color;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        ContextOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
        WindowOptions {
            title: "Draw-it example: Gradient",
            width: 800,
            height: 500,
            ..Default::default()
        },
    )?;

    let shader = context.create_shader_from_file_watch(
        "examples/gradient/shaders/gradient.shader",
        Default::default(),
    )?;

    let material = context.create_material()?;
    let mut left_color = Color::GREEN;
    let mut right_color = Color::BLUE;

    while window.is_open() {
        context.poll_events(&mut window)?;

        // update material
        material.with(|m| {
            m.set_arg_1(left_color);
            m.set_arg_2(right_color);
        });

        // draw ui
        context.draw_ui(|ui| {
            ui.auto_window("Background Control", || {
                ui.color_edit("Left Color", &mut left_color);
                ui.color_edit("Right Color", &mut right_color);
            });
        })?;

        // draw surface
        context.draw_on_window(|target| {
            target.set_shader(&shader);
            target.set_material(&material);
            target.draw_surface();
        })?;
    }

    Ok(())
}
