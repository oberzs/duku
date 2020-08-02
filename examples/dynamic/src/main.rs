// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// example with dynamicly changing mesh vertices

use draw_it::controller::Controller;
use draw_it::window::WindowOptions;
use draw_it::Color;
use draw_it::Context;
use draw_it::MeshOptions;
use draw_it::Result;
use draw_it::Vector3;
use std::time::Instant;

fn main() -> Result<()> {
    let square_size = 10;

    let (mut context, mut window) = Context::with_window(
        Default::default(),
        WindowOptions {
            title: "Draw-it example: Dynamic",
            width: 720,
            height: 640,
            ..Default::default()
        },
    )?;

    let mut controller = Controller::default();

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_backward(10.0);
        cam_t.look_at([0.0, 0.0, 0.0]);
    }

    let square = {
        let vertices = square_vertices(square_size, 0.0);
        let indices = square_indices(square_size);

        context.create_mesh(MeshOptions {
            vertices: &vertices,
            indices: &indices,
            ..Default::default()
        })?
    };
    let time = Instant::now();

    while window.is_open() {
        let stats = context.stats();
        context.poll_events(&mut window)?;
        controller.update(&mut context.main_camera, &mut window, stats.delta_time);

        // update square mesh
        square.with(|mesh| {
            let elapsed = time.elapsed().as_secs_f32();
            let vertices = square_vertices(square_size, elapsed);
            mesh.set_vertices(&vertices);
        });

        context.draw_on_window(|target| {
            target.set_wireframes(true);
            target.set_clear(Color::ORANGE);

            // draw square
            let offset = -(square_size as f32 / 2.0);
            target.draw(&square, [offset, offset, 0.0]);
        })?;
    }

    Ok(())
}

fn square_indices(size: u32) -> Vec<u32> {
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

fn square_vertices(size: u32, time: f32) -> Vec<Vector3> {
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
