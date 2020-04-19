use tegne::Tegne;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::new(640, 480);

    let tegne = Tegne::builder().with_window(&window).with_vsync().build();
    let _texture = tegne.create_texture_from_rgba(&[255, 255, 255, 255], 1, 1);

    window.start_loop(|| {});
}
