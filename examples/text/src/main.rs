use draw_it::camera::CameraType;
use draw_it::color::colors;
use draw_it::math::Transform;
use draw_it::math::Vector3;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;

fn main() {
    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Text",
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    );

    let transform = Transform {
        position: Vector3::new(-210.0, 0.0, 0.0),
        scale: Vector3::new(50.0, 50.0, 50.0),
        ..Default::default()
    };

    window.main_loop(|_, _| {
        context.draw_on_window(|target| {
            target.set_clear(colors::ORANGE);
            target.draw_text("Hello there!", transform);
        });
    });
}
