mod cube;
mod floor;

use tegne::Camera;
use tegne::Controller;
use tegne::Tegne;
use tegne::Vector3;
use tegne::Window;

use cube::Cube;
use floor::Floor;

fn main() {
    pretty_env_logger::init();

    let (width, height) = (720, 640);

    let window = Window::builder()
        .with_title("Tegne example")
        .with_size(width, height)
        .build();
    let tegne = Tegne::builder()
        .with_window(&window)
        .with_anisotropy(16.0)
        .with_msaa(4)
        .with_vsync()
        .build();

    let floor = Floor::new(&tegne);
    let cube = Cube::new(&tegne, [0.0, 0.0, 0.0], 1.0);

    let mut controller = Controller::default();

    let mut camera = Camera::perspective(width, height, 90);
    {
        let transform = camera.transform_mut();
        transform.move_by([1.0, 1.0, 3.0]);
        transform.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let blue_light = [1.0, -1.0, 1.0];
    let yellow_light = [-1.0, -1.0, -1.0];

    window.start_loop(|events| {
        controller.update(&mut camera, events);

        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([0.7, 0.7, 0.7]);
            target.add_directional_light(blue_light, [0.5, 0.5, 1.0]);
            target.add_directional_light(yellow_light, [1.0, 1.0, 0.5]);
            floor.draw(target);
            cube.draw(target);
        });
        tegne.end_draw();
    });
}
