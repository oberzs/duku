// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Mesh drawing example

mod cube;
mod floor;
mod ui;

use tegne::colors;
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
    let cube_1 = Cube::new(&tegne, [0.0, 0.0, 0.0], 1.0, colors::RED);
    let cube_2 = Cube::new(&tegne, [-3.0, 0.0, -3.0], 3.0, colors::BLUE);
    let cube_3 = Cube::new(&tegne, [-1.0, 3.0, 0.0], 1.0, colors::GREEN);
    let ui = Ui::new(&tegne, width, height);

    {
        let cam_t = &mut tegne.main_camera.transform;
        cam_t.move_by([0.0, 5.0, -10.0]);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut controller = Controller::default();

    window.main_loop(|events, _| {
        controller.update(&mut tegne.main_camera, events);

        ui.draw_ui(&mut tegne, events);

        tegne.draw_on_window(|target| {
            target.add_directional_light([-1.0, -2.0, -1.0], [1.0, 1.0, 1.0]);
            floor.draw(target);
            cube_1.draw(target);
            cube_2.draw(target);
            cube_3.draw(target);
            target.blit_framebuffer(ui.framebuffer());
        });
    });
}
