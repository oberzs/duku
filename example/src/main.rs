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
    let cube_1 = Cube::new(&tegne, [0.0, 0.0, 0.0], 1.0, "yellow");
    let cube_2 = Cube::new(&tegne, [1.0, 0.0, 1.0], 3.0, "blue1");
    let cube_3 = Cube::new(&tegne, [-2.0, 3.0, -1.0], 1.0, "blue2");

    let mut controller = Controller::default();

    let mut camera = Camera::perspective(width, height, 90);
    {
        let transform = camera.transform_mut();
        transform.move_by([1.0, 1.0, 3.0]);
        transform.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut light_pos = 0.0f32;
    let light_speed = 0.5;

    window.start_loop(|events| {
        controller.update(&mut camera, events);

        let light_x = light_pos.sin();
        let light_z = light_pos.cos();
        light_pos += light_speed * events.delta_time();

        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([0.7, 0.7, 0.7]);
            target.add_directional_light([light_x, -1.0, light_z], [1.0, 1.0, 1.0]);
            floor.draw(target);
            cube_1.draw(target);
            cube_2.draw(target);
            cube_3.draw(target);
        });
        tegne.end_draw();
    });
}
