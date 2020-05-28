// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Mesh drawing example

mod cube;
mod floor;
mod ui;

use tegne::Camera;
use tegne::Controller;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

use cube::Cube;
use floor::Floor;
use ui::Ui;

fn main() {
    pretty_env_logger::init();

    let (width, height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Cubes",
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
    let floor = Floor::new(&tegne);
    let cube_1 = Cube::new(&tegne, [0.0, 0.0, 0.0], 1.0, [1.0, 0.0, 0.0, 1.0]);
    let cube_2 = Cube::new(&tegne, [-3.0, 0.0, -3.0], 3.0, [0.0, 1.0, 0.0, 1.0]);
    let cube_3 = Cube::new(&tegne, [-1.0, 3.0, 0.0], 1.0, [0.0, 0.0, 1.0, 1.0]);
    let ui = Ui::new(&tegne, width, height);

    let mut controller = Controller::default();

    let mut camera = Camera::perspective(width, height, 90);
    {
        let transform = camera.transform_mut();
        transform.move_by([0.0, 5.0, -10.0]);
        transform.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    window.start_loop(|events| {
        controller.update(&mut camera, events);

        tegne.begin_draw();

        ui.draw_ui(&tegne, events);

        tegne.draw_on_window(&camera, |target| {
            target.add_directional_light([-1.0, -2.0, -1.0], [1.0, 1.0, 1.0]);
            floor.draw(target);
            cube_1.draw(target);
            cube_2.draw(target);
            cube_3.draw(target);
            target.blit_framebuffer(ui.framebuffer());
        });

        tegne.end_draw();
    });
}
