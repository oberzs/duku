use tegne::Id;
use tegne::Mesh;
use tegne::MeshOptions;
use tegne::Shader;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Cube {
    mesh: Vec<Id<Mesh>>,
    texture: Id<Texture>,
    shader: Id<Shader>,
    transform: Transform,
}

impl Cube {
    pub fn new(tegne: &Tegne, pos: impl Into<Vector3>, size: f32, color: impl AsRef<str>) -> Self {
        let mesh = cube(tegne, size);
        let texture = tegne
            .create_texture_from_file(format!("example/assets/images/{}.png", color.as_ref()))
            .expect("cannot create cube texture");
        let shader = tegne
            .create_shader_from_file_watch("example/assets/test.shader", Default::default())
            .expect("cannot create cube shader");
        let transform = Transform::from(pos.into());

        Self {
            mesh,
            texture,
            shader,
            transform,
        }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_shader(self.shader);
        target.set_albedo_texture(self.texture);
        for mesh in self.mesh.iter() {
            target.draw(*mesh, self.transform);
        }
        target.set_shader_phong();
    }
}

fn cube(tegne: &Tegne, size: f32) -> Vec<Id<Mesh>> {
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

    vec![top, bottom, front, back, left, right]
}

fn rectangle<V: Into<Vector3>>(tegne: &Tegne, p1: V, p2: V, p3: V, p4: V) -> Id<Mesh> {
    let vertices = &[p1.into(), p2.into(), p3.into(), p4.into()];
    let uvs = &[
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    let triangles = &[0, 2, 1, 0, 3, 2];

    tegne.create_mesh(MeshOptions {
        vertices,
        triangles,
        uvs,
        ..Default::default()
    })
}
