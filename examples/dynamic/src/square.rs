// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// grid mesh struct with changing vertices

use std::time::Instant;
use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Target;
use tegne::Tegne;
use tegne::Transform;
use tegne::Vector3;

pub struct Square {
    mesh: Mesh,
    size: u32,
    time: Instant,
}

impl Square {
    pub fn new(tegne: &mut Tegne) -> Self {
        let size = 10;

        Self {
            mesh: square(tegne, size),
            size,
            time: Instant::now(),
        }
    }

    pub fn update(&self) {
        let time = self.time.elapsed().as_secs_f32();
        let vertices = square_vertices(self.size, time);

        self.mesh.with(|mesh| {
            mesh.set_vertices(&vertices);
        });
    }

    pub fn draw(&self, target: &mut Target) {
        let offset = -(self.size as f32 / 2.0);
        target.draw(&self.mesh, Transform::from([offset, offset, 0.0]));
    }
}

fn square(tegne: &mut Tegne, size: u32) -> Mesh {
    let vertices = square_vertices(size, 0.0);
    let triangles = square_triangles(size);

    tegne.create_mesh(MeshOptions {
        vertices: &vertices,
        triangles: &triangles,
        ..Default::default()
    })
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
