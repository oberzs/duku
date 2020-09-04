// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example with dynamicly changing mesh vertices

use draw_it::window::Controller;
use draw_it::Camera;
use draw_it::Color;
use draw_it::Context;
use draw_it::Result;
use draw_it::Vector3;
use std::time::Instant;

fn main() -> Result<()> {
    let square_size = 10;

    let (mut context, mut window) = Context::builder()
        .build_window(720, 640)
        .title("Draw-it example: Dynamic")
        .build()?;

    let mut controller = Controller::fly();
    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_backward(10.0);
    camera.transform.look_at((0.0, 0.0, 0.0));

    let mut square = context
        .build_mesh()
        .vertices(square_vertices(square_size, 0.0))
        .indices(square_indices(square_size))
        .build();
    let time = Instant::now();

    while window.is_open() {
        context.poll_events(&mut window);
        controller.update(&mut camera, &mut window, context.delta_time());

        // update square mesh
        let elapsed = time.elapsed().as_secs_f32();
        square.vertices = square_vertices(square_size, elapsed);
        square.update();

        context.draw_on_window(Some(&camera), |target| {
            target.wireframes = true;
            target.clear = Color::ORANGE;

            // draw square
            let offset = -(square_size as f32 / 2.0);
            target.draw(&square, (offset, offset));
        });
    }

    Ok(())
}

fn square_indices(size: u16) -> Vec<u16> {
    let mut indices = Vec::with_capacity(size as usize * size as usize * 2);
    let mut vi = 0;
    for _ in 0..size {
        for _ in 0..size {
            indices.extend(&[
                vi,
                vi + size + 1,
                vi + 1,
                vi + 1,
                vi + size + 1,
                vi + size + 2,
            ]);
            vi += 1;
        }
        vi += 1;
    }
    indices
}

fn square_vertices(size: u16, time: f32) -> Vec<Vector3> {
    let mut vertices = Vec::with_capacity((size as usize + 1) * (size as usize + 1));
    for y in 0..=size {
        for x in 0..=size {
            let xx = x as f32;
            let yy = y as f32;
            vertices.push(Vector3::new(
                xx + (yy + time * 1.5).cos() * 0.3,
                yy + (xx + time * 1.5).sin() * 0.3,
                0.0,
            ));
        }
    }
    vertices
}
