// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Floor mesh struct with custom texture

use tegne::Mesh;
use tegne::MeshOptions;
use tegne::SamplerFilter;
use tegne::SamplerOptions;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Floor {
    mesh: Mesh,
    texture: Texture,
}

impl Floor {
    pub fn new(tegne: &mut Tegne) -> Self {
        let mesh = plane(tegne, 150.0);
        let texture = tegne
            .create_texture_from_file("examples/cubes/textures/Light/texture_08.png")
            .unwrap();

        Self { mesh, texture }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_albedo_texture(&self.texture);
        target.set_sampler(SamplerOptions {
            filter: SamplerFilter::Nearest,
            ..Default::default()
        });
        target.draw(&self.mesh, Transform::default());
        target.set_sampler(Default::default());
    }
}

fn plane(tegne: &mut Tegne, size: f32) -> Mesh {
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
