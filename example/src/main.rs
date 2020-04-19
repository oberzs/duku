use tegne::Tegne;
use tegne::Vector2;
use tegne::Vector3;
use tegne::Window;

fn main() {
    pretty_env_logger::init();

    let window = Window::new(640, 480);

    let tegne = Tegne::builder().with_window(&window).with_vsync().build();
    let texture = tegne.create_texture_from_rgba(&[255, 255, 255, 255], 1, 1);
    let _material = tegne.create_material().with_albedo(texture).build();

    let vertices = &[
        Vector3::new(-0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, -0.5),
        Vector3::new(0.5, -0.5, 0.5),
    ];
    let triangles = &[0, 1, 2];
    let uvs = &[
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 1.0),
        Vector2::new(0.0, 1.0),
    ];

    let _mesh = tegne
        .create_mesh()
        .with_vertices(vertices)
        .with_triangles(triangles)
        .with_uvs(uvs)
        .with_smooth_normals()
        .build();

    window.start_loop(|| {});
}
