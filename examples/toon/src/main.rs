// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Toon shader example

use tegne::colors;
use tegne::Controller;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Transform;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Toon",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(
        &mut window,
        TegneOptions {
            anisotropy: 16.0,
            msaa: 4,
            ..Default::default()
        },
    );

    let texture = tegne
        .create_texture_from_file("examples/toon/textures/texture_09.png")
        .unwrap();

    let shader = tegne
        .create_shader_from_file_watch("examples/toon/shaders/toon.shader", Default::default())
        .unwrap();

    {
        let cam_t = &mut tegne.main_camera.transform;
        cam_t.move_backward(5.0);
        cam_t.move_up(2.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut controller = Controller::default();

    let floor_transform = Transform {
        scale: Vector3::new(10.0, 0.2, 10.0),
        ..Default::default()
    };

    window.main_loop(|events, _| {
        controller.update(&mut tegne.main_camera, events);

        tegne.draw_on_window(|target| {
            target.set_clear(colors::SKY_BLUE);

            // floor
            target.draw_cube(floor_transform);

            // toon cube and sphere
            target.set_shader(&shader);
            target.draw_cube([-3.0, 1.0, 0.0]);
            target.draw_sphere([-1.0, 1.0, 0.0]);
            target.set_shader_phong();

            // textured cube and sphere
            target.set_albedo(&texture);
            target.draw_cube([1.0, 1.0, 0.0]);
            target.draw_sphere([3.0, 1.0, 0.0]);
        });
    });
}
