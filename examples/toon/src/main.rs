// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Toon shader example

use draw_it::camera::Controller;
use draw_it::color::colors;
use draw_it::math::Transform;
use draw_it::math::Vector3;
use draw_it::ui;
use draw_it::ui::label;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;

fn main() {
    let (width, height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Toon",
        width,
        height,
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            anisotropy: 16.0,
            msaa: 4,
            vsync: false,
            ..Default::default()
        },
    );

    let texture = context
        .create_texture_from_file("examples/toon/textures/texture_09.png")
        .unwrap();

    let shader = context
        .create_shader_from_file_watch("examples/toon/shaders/toon.shader", Default::default())
        .unwrap();

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_backward(5.0);
        cam_t.move_up(2.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut controller = Controller::default();

    let floor_transform = Transform {
        scale: Vector3::new(10.0, 0.2, 10.0),
        ..Default::default()
    };
    let mut shadow_softness = 1.0;

    window.main_loop(|events, ui| {
        controller.update(&mut context.main_camera, events);

        ui::stats_window(&ui, &context, events);
        ui::Window::new(label!("Shadow Settings"))
            .size([1.0, 1.0], ui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .build(&ui, || {
                ui.drag_float(label!("Softness"), &mut shadow_softness)
                    .min(0.0)
                    .speed(0.1)
                    .build();
            });
        context.draw_ui(ui);

        context.draw_on_window(|target| {
            target.set_clear(colors::SKY_BLUE);
            target.set_shadow_softness(shadow_softness);

            // floor
            target.draw_cube(floor_transform);

            // toon cube and sphere
            target.set_shader(&shader);
            target.draw_cube([-3.0, 0.6, 0.0]);
            target.draw_sphere([-1.0, 1.0, 0.0]);
            target.set_shader_phong();

            // textured cube and sphere
            target.set_albedo(&texture);
            target.draw_cube([1.0, 1.0, 0.0]);
            target.draw_sphere([3.0, 1.0, 0.0]);
        });
    });
}
