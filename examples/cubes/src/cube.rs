// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Cube mesh struct with texture

use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Target;
use tegne::Tegne;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Cube {
    mesh: Mesh,
    transform: Transform,
}

impl Cube {
    pub fn new(tegne: &Tegne, pos: impl Into<Vector3>, size: f32) -> Self {
        let mesh = cube(tegne, size);
        let transform = Transform::from(pos.into());

        Self { mesh, transform }
    }

    pub fn draw(&self, target: &mut Target) {
        target.draw(&self.mesh, self.transform);
    }
}

fn cube(tegne: &Tegne, size: f32) -> Mesh {
    let top = rectangle(
        tegne,
        [0.0, size, 0.0],
        [size, size, 0.0],
        [size, size, size],
        [0.0, size, size],
    );

    let bottom = rectangle(
        tegne,
        [0.0, 0.0, 0.0],
        [0.0, 0.0, size],
        [size, 0.0, size],
        [size, 0.0, 0.0],
    );

    let front = rectangle(
        tegne,
        [0.0, size, size],
        [size, size, size],
        [size, 0.0, size],
        [0.0, 0.0, size],
    );

    let back = rectangle(
        tegne,
        [0.0, size, 0.0],
        [0.0, 0.0, 0.0],
        [size, 0.0, 0.0],
        [size, size, 0.0],
    );

    let left = rectangle(
        tegne,
        [0.0, size, 0.0],
        [0.0, size, size],
        [0.0, 0.0, size],
        [0.0, 0.0, 0.0],
    );

    let right = rectangle(
        tegne,
        [size, size, size],
        [size, size, 0.0],
        [size, 0.0, 0.0],
        [size, 0.0, size],
    );

    tegne.combine_meshes(&[top, bottom, front, back, left, right])
}

fn rectangle<V: Into<Vector3>>(tegne: &Tegne, p1: V, p2: V, p3: V, p4: V) -> Mesh {
    let vertices = &[p1.into(), p2.into(), p3.into(), p4.into()];
    let uvs = &[
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    let triangles = &[[0, 2, 1], [0, 3, 2]];

    tegne.create_mesh(MeshOptions {
        vertices,
        triangles,
        uvs,
        ..Default::default()
    })
}
