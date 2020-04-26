use tegne::Tegne;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let mut window = Window::new(480, 360);
    let tegne = Tegne::builder().with_window(&window).with_vsync().build();

    window.start_loop(|| {
        tegne.draw_on_window(|target| {
            target.draw_cube([0.0, 0.0, 0.0]);
        });
    });
}
