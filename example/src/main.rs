use tegne::Tegne;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let mut window = Window::new(400, 400);
    let tegne = Tegne::builder().with_window(&window).with_vsync().build();

    window.start_loop(|| {
        tegne.begin_draw();
        tegne.draw_on_window(|target| {
            target.set_clear_color([0.0, 1.0, 1.0]);
            target.draw_cube([0.0, 0.0, 0.0]);
        });
        tegne.end_draw();
    });
}
