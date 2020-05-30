// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Floor mesh struct with custom texture

use tegne::Id;
use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Floor {
    mesh: Id<Mesh>,
    texture: Id<Texture>,
}

impl Floor {
    pub fn new(tegne: &Tegne) -> Self {
        let mesh = plane(tegne, 100.0);
        let texture = tegne.create_texture_from_file("examples/cubes/assets/images/grey2.png");

        Self { mesh, texture }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_albedo_texture(&self.texture);
        target.draw(&self.mesh, Transform::default());
    }
}

fn plane(tegne: &Tegne, size: f32) -> Id<Mesh> {
    let half_size = size / 2.0;
    let vertices = &[
        Vector3::new(-half_size, 0.0, half_size),
        Vector3::new(half_size, 0.0, half_size),
        Vector3::new(half_size, 0.0, -half_size),
        Vector3::new(-half_size, 0.0, -half_size),
    ];
    let uvs = &[
        Vector2::new(0.0, 0.0),
        Vector2::new(size, 0.0),
        Vector2::new(size, size),
        Vector2::new(0.0, size),
    ];
    let triangles = &[[0, 1, 2], [0, 2, 3]];

    tegne.create_mesh(MeshOptions {
        vertices,
        triangles,
        uvs,
        ..Default::default()
    })
}
