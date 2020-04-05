use tegne::Tegne;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::new();

    let _tegne = Tegne::from_window(&window);

    window.start_loop(|| {});
}
