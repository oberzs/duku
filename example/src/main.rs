use tegne::Tegne;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::new(640, 480);

    let _tegne = Tegne::builder().with_window(&window).with_vsync().build();

    window.start_loop(|| {});
}
