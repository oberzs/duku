use std::collections::HashMap;
use std::f32::consts::PI;
use std::rc::Rc;
use tegne_math::Vector2;
use tegne_math::Vector3;

use crate::instance::Device;
use crate::model::Mesh;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) enum BuiltinMesh {
    Cube,
    Sphere,
}

pub(crate) fn builtin_meshes(device: &Rc<Device>) -> HashMap<BuiltinMesh, Mesh> {
    let mut map = HashMap::new();
    map.insert(BuiltinMesh::Cube, create_cube(device));
    map.insert(BuiltinMesh::Sphere, create_sphere(device, 2));
    map
}

fn create_cube(device: &Rc<Device>) -> Mesh {
    let vertices = vec![
        // bottom
        Vector3::new(-0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, 0.5),
        Vector3::new(-0.5, -0.5, 0.5),
        // top
        Vector3::new(-0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, -0.5),
        Vector3::new(0.5, 0.5, 0.5),
        Vector3::new(-0.5, 0.5, 0.5),
    ];
    let uvs = vec![
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(1.0, 0.0),
    ];
    let triangles = vec![
        0, 1, 2, 0, 2, 3, // bottom
        4, 7, 6, 4, 6, 5, // top
        4, 5, 1, 4, 1, 0, // front
        7, 3, 2, 7, 2, 6, // back
        5, 6, 2, 5, 2, 1, // right
        7, 4, 0, 7, 0, 3, // left
    ];

    Mesh::builder(device)
        .with_vertices(&vertices)
        .with_triangles(&triangles)
        .with_uvs(&uvs)
        .with_smooth_normals()
        .build()
}

fn create_sphere(device: &Rc<Device>, detail_level: u32) -> Mesh {
    let mut vertices = vec![];
    let mut triangles = vec![];

    // 12 icosahedron vertices
    let t = (1.0 + 5.0f32.sqrt()) / 2.0;

    vertices.push(Vector3::new(-1.0, t, 0.0).unit());
    vertices.push(Vector3::new(1.0, t, 0.0).unit());
    vertices.push(Vector3::new(-1.0, -t, 0.0).unit());
    vertices.push(Vector3::new(1.0, -t, 0.0).unit());

    vertices.push(Vector3::new(0.0, -1.0, t).unit());
    vertices.push(Vector3::new(0.0, 1.0, t).unit());
    vertices.push(Vector3::new(0.0, -1.0, -t).unit());
    vertices.push(Vector3::new(0.0, 1.0, -t).unit());

    vertices.push(Vector3::new(t, 0.0, -1.0).unit());
    vertices.push(Vector3::new(t, 0.0, 1.0).unit());
    vertices.push(Vector3::new(-t, 0.0, -1.0).unit());
    vertices.push(Vector3::new(-t, 0.0, 1.0).unit());

    // 20 icosahedron triangles
    triangles.extend(&[0, 11, 5]);
    triangles.extend(&[0, 5, 1]);
    triangles.extend(&[0, 1, 7]);
    triangles.extend(&[0, 7, 10]);
    triangles.extend(&[0, 10, 11]);

    triangles.extend(&[1, 5, 9]);
    triangles.extend(&[5, 11, 4]);
    triangles.extend(&[11, 10, 2]);
    triangles.extend(&[10, 7, 6]);
    triangles.extend(&[7, 1, 8]);

    triangles.extend(&[3, 9, 4]);
    triangles.extend(&[3, 4, 2]);
    triangles.extend(&[3, 2, 6]);
    triangles.extend(&[3, 6, 8]);
    triangles.extend(&[3, 8, 9]);

    triangles.extend(&[4, 9, 5]);
    triangles.extend(&[2, 4, 11]);
    triangles.extend(&[6, 2, 10]);
    triangles.extend(&[8, 6, 7]);
    triangles.extend(&[9, 8, 1]);

    // refine triangles
    let mut midpoints = HashMap::new();
    for _ in 0..detail_level {
        let mut new_triangles = vec![];
        triangles.chunks(3).for_each(|tri| {
            // replace triangle with 4 triangles
            let a = get_middle_point(&mut vertices, tri[0], tri[1], &mut midpoints);
            let b = get_middle_point(&mut vertices, tri[1], tri[2], &mut midpoints);
            let c = get_middle_point(&mut vertices, tri[2], tri[0], &mut midpoints);

            new_triangles.extend(&[tri[0], a, c]);
            new_triangles.extend(&[tri[1], b, a]);
            new_triangles.extend(&[tri[2], c, b]);
            new_triangles.extend(&[a, b, c]);
        });
        triangles = new_triangles;
    }

    let mut uvs = vec![];
    for vertex in vertices.iter() {
        let u = vertex.z.atan2(vertex.x) / (2.0 * PI);
        let v = (vertex.y.asin() / PI) + 0.5;
        uvs.push(Vector2::new(u, v));
    }

    Mesh::builder(device)
        .with_vertices(&vertices)
        .with_triangles(&triangles)
        .with_uvs(&uvs)
        .with_smooth_normals()
        .build()
}

fn get_middle_point(
    vertices: &mut Vec<Vector3>,
    p1: u32,
    p2: u32,
    midpoints: &mut HashMap<(u32, u32), u32>,
) -> u32 {
    match (midpoints.get(&(p1, p2)), midpoints.get(&(p2, p1))) {
        (Some(i), _) => *i,
        (_, Some(i)) => *i,
        (None, None) => {
            let point_1 = vertices[p1 as usize];
            let point_2 = vertices[p2 as usize];
            let middle = (point_1 + point_2) / 2.0;

            vertices.push(middle.unit());
            let i = vertices.len() as u32 - 1;
            midpoints.insert((p1, p2), i);
            i
        }
    }
}
