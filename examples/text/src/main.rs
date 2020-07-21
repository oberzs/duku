use draw_it::camera::CameraType;
use draw_it::color::colors;
use draw_it::math::Transform;
use draw_it::math::Vector3;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;
use std::time::Instant;

fn main() {
    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Text",
        width: 600,
        height: 400,
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    );

    let start_time = Instant::now();
    let left = -250.0;

    window.main_loop(|_, _| {
        let size_mut = (start_time.elapsed().as_secs_f32().sin() * 0.5 + 0.5) * 10.0;
        let sdf_size = 40 + size_mut.round() as u32;
        let dynamic_size = 1.0 + (start_time.elapsed().as_secs_f32().sin() * 0.5 + 0.5);

        context.draw_on_window(|target| {
            target.set_clear(colors::ORANGE);
            target.set_font_size(sdf_size);
            target.draw_text(
                format!("SDF {}p text\n... in two lines!", sdf_size),
                [left, 200.0, 1.0],
            );
            target.set_font_size(32);
            target.draw_text("Bitmap 32p text", [left, 100.0, 1.0]);
            target.set_font_size(24);
            target.draw_text("Bitmap 24p text", [left, 68.0, 1.0]);
            target.set_font_size(18);
            target.draw_text("Bitmap 18p text", [left, 44.0, 1.0]);
            target.set_font_size(40);
            let transform = Transform {
                position: Vector3::new(left, 0.0, 1.0),
                scale: Vector3::new(dynamic_size, dynamic_size, dynamic_size),
                ..Default::default()
            };
            target.draw_text("Dynamic\n-text-", transform);
        });
    });
}
