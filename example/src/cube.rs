use tegne::read_image;
use tegne::Material;
use tegne::Mesh;
use tegne::Target;
use tegne::Tegne;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector2;
use tegne::Vector3;

pub struct Cube {
    mesh: Vec<Mesh>,
    material: Material,
    _texture: Texture,
    transform: Transform,
}

impl Cube {
    pub fn new(tegne: &Tegne, pos: impl Into<Vector3>, size: f32) -> Self {
        let mesh = cube(tegne, size);
        let (image, width, height) = read_image("example/assets/prototype_512x512_yellow.png");
        let texture = tegne.create_texture_rgba(&image, width, height);
        let material = tegne.create_material().with_albedo(&texture).build();
        let transform = Transform::builder().with_position(pos).build();

        Self {
            mesh,
            material,
            _texture: texture,
            transform,
        }
    }

    pub fn draw(&self, target: &mut Target) {
        target.set_material(&self.material);
        for mesh in self.mesh.iter() {
            target.draw(mesh, self.transform);
        }
    }
}

fn cube(tegne: &Tegne, size: f32) -> Vec<Mesh> {
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

fn rectangle<V: Into<Vector3>>(tegne: &Tegne, p1: V, p2: V, p3: V, p4: V) -> Mesh {
    let vertices = vec![p1.into(), p2.into(), p3.into(), p4.into()];
    let uvs = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];
    let triangles = vec![0, 2, 1, 0, 3, 2];

    tegne
        .create_mesh()
        .with_vertices(&vertices)
        .with_uvs(&uvs)
        .with_triangles(&triangles)
        .with_smooth_normals()
        .build()
}
