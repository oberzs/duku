// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example that draws text

use draw_it::window::WindowOptions;
use draw_it::CameraType;
use draw_it::Color;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Quality;
use draw_it::Result;
use draw_it::Transform;
use draw_it::Vector3;
use std::time::Instant;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        ContextOptions {
            quality: Quality::Low,
            camera: CameraType::Orthographic,
            ..Default::default()
        },
        WindowOptions {
            title: "Draw-it example: Text",
            width: 600,
            height: 400,
            ..Default::default()
        },
    )?;

    let material_1 = context.create_material()?;
    material_1.with(|m| {
        m.set_font_color(Color::RED);
    });
    let material_2 = context.create_material()?;
    material_2.with(|m| {
        m.set_font_color(Color::BLUE);
        m.set_font_border_color(Color::WHITE);
        m.set_font_width(0.5);
        m.set_font_edge(0.1);
        m.set_font_border_width(0.8);
        m.set_font_border_edge(0.1);
    });

    let start_time = Instant::now();
    let left = -250.0;

    while window.is_open() {
        context.poll_events(&mut window)?;

        let size_mut = (start_time.elapsed().as_secs_f32().sin() * 0.5 + 0.5) * 10.0;
        let sdf_size = 40 + size_mut.round() as u32;
        let dynamic_size = 1.0 + (start_time.elapsed().as_secs_f32().sin() * 0.5 + 0.5);

        context.draw_on_window(|target| {
            target.set_clear(Color::ORANGE);
            target.set_font_size(sdf_size);
            target.draw_text(
                format!("SDF {}p text\n... in two lines!", sdf_size),
                [left, 200.0, 1.0],
            );
            target.set_font_size(32);
            target.set_font_material(&material_1);
            target.draw_text("Bitmap 32p text", [left, 100.0, 1.0]);
            target.set_font_material_black();
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
            target.set_font_material(&material_2);
            target.draw_text("Dynamic\n-text-", transform);
        })?;
    }

    Ok(())
}
