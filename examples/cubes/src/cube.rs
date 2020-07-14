// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Cube mesh struct with texture

use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Cube {
    mesh: Mesh,
    texture: Texture,
    transform: Transform,
}

impl Cube {
    pub fn new(tegne: &mut Tegne, texture: &Texture, pos: impl Into<Vector3>) -> Self {
        let mesh = cube(tegne, 1.0);
        let transform = Transform::from(pos.into());

        Self {
            texture: texture.clone(),
            mesh,
            transform,
        }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_albedo(&self.texture);
        target.draw(&self.mesh, self.transform);
    }
}

fn cube(tegne: &mut Tegne, size: f32) -> Mesh {
    let top = rectangle(
        tegne,
        [0.0, size, size],
        [size, size, size],
        [size, size, 0.0],
        [0.0, size, 0.0],
    );

    let bottom = rectangle(
        tegne,
        [size, 0.0, size],
        [0.0, 0.0, size],
        [0.0, 0.0, 0.0],
        [size, 0.0, 0.0],
    );

    let back = rectangle(
        tegne,
        [size, size, size],
        [0.0, size, size],
        [0.0, 0.0, size],
        [size, 0.0, size],
    );

    let front = rectangle(
        tegne,
        [0.0, size, 0.0],
        [size, size, 0.0],
        [size, 0.0, 0.0],
        [0.0, 0.0, 0.0],
    );

    let left = rectangle(
        tegne,
        [0.0, size, size],
        [0.0, size, 0.0],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, size],
    );

    let right = rectangle(
        tegne,
        [size, size, 0.0],
        [size, size, size],
        [size, 0.0, size],
        [size, 0.0, 0.0],
    );

    tegne.combine_meshes(&[top, bottom, front, back, left, right])
}

fn rectangle<V: Into<Vector3>>(tegne: &mut Tegne, p1: V, p2: V, p3: V, p4: V) -> Mesh {
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
