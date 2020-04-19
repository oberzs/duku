use tegne::Tegne;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::new(640, 480);

    let _tegne = Tegne::from_window(&window, 0);

    window.start_loop(|| {});
}
