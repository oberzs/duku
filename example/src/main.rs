use tegne::Camera;
use tegne::Controller;
use tegne::Tegne;
use tegne::Vector3;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::builder()
        .with_title("Tegne example")
        .with_size(640, 480)
        .build();
    let tegne = Tegne::builder()
        .with_window(&window)
        .with_msaa(2)
        .with_vsync()
        .build();

    let mut controller = Controller::default();

    let mut camera = Camera::perspective(640, 480, 90);
    {
        let transform = camera.transform_mut();
        transform.move_by([1.0, 1.0, 3.0]);
        transform.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut light_pos = 0.0f32;
    let light_speed = 0.01;

    window.start_loop(move |events| {
        controller.update(&mut camera, events);

        let light_x = light_pos.cos();
        let light_z = light_pos.sin();
        light_pos += light_speed;

        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([0.7, 0.7, 0.7]);
            target.add_directional_light([light_x, -1.0, light_z], [1.0, 1.0, 1.0]);
            target.draw_cube([0.0, 0.0, 0.0]);
        });
        tegne.end_draw();
    });
}
