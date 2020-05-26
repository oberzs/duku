// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Cube mesh struct with custom shader

use tegne::Id;
use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Shader;
use tegne::Target;
use tegne::Tegne;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;
use tegne::Vector4;

pub struct Cube {
    mesh: Vec<Id<Mesh>>,
    shader: Id<Shader>,
    transform: Transform,
}

impl Cube {
    pub fn new(
        tegne: &Tegne,
        pos: impl Into<Vector3>,
        size: f32,
        color: impl Into<Vector4>,
    ) -> Self {
        let mesh = cube(tegne, size, color.into());
        let shader = tegne
            .create_shader_from_file_watch("examples/cubes/assets/test.shader", Default::default())
            .expect("cannot create cube shader");
        let transform = Transform::from(pos.into());

        Self {
            mesh,
            shader,
            transform,
        }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_shader(&self.shader);
        for mesh in self.mesh.iter() {
            target.draw(mesh, self.transform);
        }
        target.set_shader_phong();
    }
}

fn cube(tegne: &Tegne, size: f32, color: Vector4) -> Vec<Id<Mesh>> {
    let top = rectangle(
        tegne,
        color,
        [0.0, size, 0.0],
        [size, size, 0.0],
        [size, size, size],
        [0.0, size, size],
    );

    let bottom = rectangle(
        tegne,
        color,
        [0.0, 0.0, 0.0],
        [0.0, 0.0, size],
        [size, 0.0, size],
        [size, 0.0, 0.0],
    );

    let front = rectangle(
        tegne,
        color,
        [0.0, size, size],
        [size, size, size],
        [size, 0.0, size],
        [0.0, 0.0, size],
    );

    let back = rectangle(
        tegne,
        color,
        [0.0, size, 0.0],
        [0.0, 0.0, 0.0],
        [size, 0.0, 0.0],
        [size, size, 0.0],
    );

    let left = rectangle(
        tegne,
        color,
        [0.0, size, 0.0],
        [0.0, size, size],
        [0.0, 0.0, size],
        [0.0, 0.0, 0.0],
    );

    let right = rectangle(
        tegne,
        color,
        [size, size, size],
        [size, size, 0.0],
        [size, 0.0, 0.0],
        [size, 0.0, size],
    );

    vec![top, bottom, front, back, left, right]
}

fn rectangle<V: Into<Vector3>>(
    tegne: &Tegne,
    color: Vector4,
    p1: V,
    p2: V,
    p3: V,
    p4: V,
) -> Id<Mesh> {
    let vertices = &[p1.into(), p2.into(), p3.into(), p4.into()];
    let uvs = &[
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    let colors = &[color, color, color, color];
    let triangles = &[[0, 2, 1], [0, 3, 2]];

    tegne.create_mesh(MeshOptions {
        vertices,
        triangles,
        uvs,
        colors,
        ..Default::default()
    })
}
