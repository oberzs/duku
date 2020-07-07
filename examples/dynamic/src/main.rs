// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example with dynamicly changing mesh vertices

mod square;

use tegne::Controller;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

use square::Square;

fn main() {
    let (width, height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Dynamic",
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

    let mut controller = Controller::default();

    {
        let cam_t = &mut tegne.main_camera.transform;
        cam_t.move_backward(10.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let square = Square::new(&mut tegne);

    window.main_loop(|events, _| {
        controller.update(&mut tegne.main_camera, events);
        square.update();

        tegne.draw_on_window(|target| {
            target.set_wireframes(true);
            square.draw(target);
        });
    });
}
