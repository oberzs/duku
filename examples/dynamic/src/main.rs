mod square;

use tegne::Camera;
use tegne::Controller;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

use square::Square;

fn main() {
    pretty_env_logger::init();

    let (width, height) = (720, 640);

    let window = Window::new(WindowOptions {
        title: "Tegne example: Dynamic",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(
        &window,
        TegneOptions {
            anisotropy: 16.0,
            msaa: 4,
            ..Default::default()
        },
    );

    let mut controller = Controller::default();

    let mut camera = Camera::perspective(width, height, 90);
    {
        let transform = camera.transform_mut();
        transform.move_backward(10.0);
        transform.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let square = Square::new(&tegne);

    window.start_loop(|events| {
        controller.update(&mut camera, events);
        square.update(&tegne);

        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_wireframes(true);
            square.draw(target);
        });
        tegne.end_draw();
    });
}
