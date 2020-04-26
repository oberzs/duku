use tegne::Camera;
use tegne::Tegne;
use tegne::Vector3;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let mut window = Window::new(400, 400);
    let tegne = Tegne::builder().with_window(&window).with_vsync().build();

    let mut camera = Camera::perspective(400, 400, 90);
    {
        let transform = camera.transform_mut();
        transform.move_by([1.0, 1.0, 3.0]);
        transform.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    window.start_loop(|| {
        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([0.7, 0.7, 0.7]);
            target.draw_cube([0.0, 0.0, 0.0]);
        });
        tegne.end_draw();
    });
}
