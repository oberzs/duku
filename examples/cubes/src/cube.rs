// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Cube mesh struct with texture

use rand::Rng;
use tegne::Id;
use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Cube {
    mesh: Id<Mesh>,
    texture: Id<Texture>,
    transform: Transform,
}

const COLORS: [&str; 16] = [
    "blue1", "blue2", "blue3", "brown", "cyan", "green1", "green2", "grey1", "grey2", "grey3",
    "grey4", "orange", "purple", "red", "white", "yellow",
];

impl Cube {
    pub fn new(tegne: &Tegne, pos: impl Into<Vector3>, size: f32) -> Self {
        let mut rng = rand::thread_rng();
        let color = COLORS[rng.gen_range(0, 16)];

        let mesh = cube(tegne, size);
        let texture = tegne
            .create_texture_from_file(format!("examples/cubes/assets/images/{}.png", color))
            .expect("cannot open cube texture file");
        let transform = Transform::from(pos.into());

        Self {
            mesh,
            texture,
            transform,
        }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_albedo_texture(&self.texture);
        target.draw(&self.mesh, self.transform);
    }
}

fn cube(tegne: &Tegne, size: f32) -> Id<Mesh> {
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

fn rectangle<V: Into<Vector3>>(tegne: &Tegne, p1: V, p2: V, p3: V, p4: V) -> Id<Mesh> {
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
