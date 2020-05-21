use tegne::Camera;
use tegne::MaterialOptions;
use tegne::Tegne;
use tegne::Vector4;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (800, 500);
    let color_1 = Vector4::new(1.0, 0.0, 0.0, 1.0);
    let color_2 = Vector4::new(0.0, 0.0, 1.0, 1.0);

    let window = Window::new(WindowOptions {
        title: "Tegne example: Surface",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(&window, Default::default());

    let camera = Camera::orthographic(width, height);

    let shader = tegne
        .create_shader_from_file_watch(
            "examples/gradient/assets/gradient.shader",
            Default::default(),
        )
        .unwrap();
    let material = tegne.create_material(MaterialOptions {
        arg_1: color_1,
        arg_2: color_2,
        ..Default::default()
    });

    window.start_loop(|_| {
        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_shader(shader);
            target.set_material(material);
            target.draw_surface();
        });
        tegne.end_draw();
    });
}
