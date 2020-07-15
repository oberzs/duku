// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example with dynamicly changing mesh vertices

use std::time::Instant;
use tegne::camera::Controller;
use tegne::color::colors;
use tegne::math::Vector3;
use tegne::mesh::MeshOptions;
use tegne::window::Window;
use tegne::window::WindowOptions;
use tegne::Context;
use tegne::ContextOptions;

fn main() {
    let (width, height) = (720, 640);
    let square_size = 10;

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Dynamic",
        width,
        height,
        ..Default::default()
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            anisotropy: 16.0,
            msaa: 4,
            ..Default::default()
        },
    );

    let mut controller = Controller::default();

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_backward(10.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let square = {
        let vertices = square_vertices(square_size, 0.0);
        let triangles = square_triangles(square_size);

        context.create_mesh(MeshOptions {
            vertices: &vertices,
            triangles: &triangles,
            ..Default::default()
        })
    };
    let time = Instant::now();

    window.main_loop(|events, _| {
        controller.update(&mut context.main_camera, events);

        // update square mesh
        square.with(|mesh| {
            let elapsed = time.elapsed().as_secs_f32();
            let vertices = square_vertices(square_size, elapsed);
            mesh.set_vertices(&vertices);
        });

        context.draw_on_window(|target| {
            target.set_wireframes(true);
            target.set_clear(colors::ORANGE);

            // draw square
            let offset = -(square_size as f32 / 2.0);
            target.draw(&square, [offset, offset, 0.0]);
        });
    });
}

fn square_triangles(size: u32) -> Vec<[u32; 3]> {
    let mut triangles = Vec::with_capacity(size as usize * size as usize * 2);
    let mut vi = 0;
    for _ in 0..size {
        for _ in 0..size {
            triangles.push([vi, vi + size + 1, vi + 1]);
            triangles.push([vi + 1, vi + size + 1, vi + size + 2]);
            vi += 1;
        }
        vi += 1;
    }
    triangles
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
