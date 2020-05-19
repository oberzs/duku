use tegne::Camera;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (500, 500);

    let window = Window::new(WindowOptions {
        title: "Tegne example: Blue",
        width,
        height,
    });
    let mut tegne = Tegne::from_window(
        &window,
        TegneOptions {
            vsync: true,
            ..Default::default()
        },
    );

    let camera = Camera::orthographic(width, height);

    window.start_loop(|_| {
        tegne.begin_draw();
        tegne.draw_on_window(&camera, |target| {
            target.set_clear_color([0.0, 0.0, 1.0, 1.0]);
        });
        tegne.end_draw();
    });
}
