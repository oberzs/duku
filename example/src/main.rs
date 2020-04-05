use tegne::Tegne;
use tegne_utils::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::new();

    let _tegne = Tegne::new();

    window.start_loop(|| {});
}
