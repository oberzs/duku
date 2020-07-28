// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Toon shader example

use draw_it::camera::Controller;
use draw_it::error::Result;
use draw_it::math::Vector3;
use draw_it::ui;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;

fn main() -> Result<()> {
    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Toon",
        width: 720,
        height: 640,
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            vsync: false,
            ..Default::default()
        },
    )?;

    let shader = context
        .create_shader_from_file_watch("examples/toon/shaders/toon.shader", Default::default())?;

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_backward(5.0);
        cam_t.move_up(2.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut controller = Controller::default();

    window.main_loop(|events, ui| {
        controller.update(&mut context.main_camera, events);

        ui::stats_window(&ui, &context, events);
        context.draw_ui(ui)?;

        context.draw_on_window(|target| {
            target.set_clear([128, 128, 128]);

            // grid
            target.draw_grid();

            // toon cube and sphere
            target.set_shader(&shader);
            target.draw_sphere([0.0, 1.0, 0.0]);
        })?;

        Ok(())
    });

    Ok(())
}
