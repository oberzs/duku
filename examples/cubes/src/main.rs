// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh drawing example

use draw_it::window::Controller;
use draw_it::window::Event;
use draw_it::window::WindowOptions;
use draw_it::Camera;
use draw_it::Color;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Light;
use draw_it::Mesh;
use draw_it::Quaternion;
use draw_it::Result;
use draw_it::Transform;
use draw_it::VSync;
use draw_it::Vector2;
use draw_it::Vector3;

fn main() -> Result<()> {
    let (width, height) = (720, 640);

    let (mut context, mut window) = Context::with_window(
        ContextOptions {
            vsync: VSync::Off,
            ..Default::default()
        },
        WindowOptions {
            title: "Draw-it example: Cubes",
            resizable: true,
            width,
            height,
        },
    )?;

    let mut camera = Camera::perspective(width as f32, height as f32, 90);
    camera.transform.move_by([1.0, 3.0, -3.0]);
    camera.transform.look_dir(Vector3::FORWARD);

    let mut controller = Controller::orbit([0.0, 0.0, 0.0]);

    context.set_skybox_from_file("examples/cubes/textures/Skybox/glacier_up.cubemap")?;

    let cube = cube_mesh(&mut context, [1.0, 1.0, 1.0])?;

    let floor_transform = Transform {
        scale: Vector3::new(80.0, 0.2, 80.0),
        position: Vector3::new(0.0, -0.1, 0.0),
        ..Default::default()
    };

    let main_light = Light::directional([-0.4, -1.0, -1.0], Color::WHITE, true);

    while window.is_open() {
        // update
        context.poll_events(&mut window)?;

        for event in window.events() {
            let Event::Resize(w, h) = event;
            camera.width = w as f32;
            camera.height = h as f32;
        }

        let delta_time = context.delta_time();
        controller.update(&mut camera, &mut window, delta_time);

        // render
        let stats = context.stats();
        let fps = context.fps();
        context.draw_ui(|ui| {
            ui.stats_window(stats, fps, delta_time);
        })?;

        context.draw_on_window(&camera, |target| {
            target.skybox = true;
            target.lights[0] = main_light;
            // target.draw_grid();
            target.draw_cube(floor_transform);
            target.draw(&cube, [2.0, 1.0, 0.0]);
            target.draw_cube([0.0, 0.0, 0.0]);
            target.draw_cube([-2.0, 1.0, 0.0]);
            target.draw_sphere([-4.0, 1.0, 0.0]);
        })?;
    }

    Ok(())
}

fn cube_mesh(context: &mut Context, size: impl Into<Vector3>) -> Result<Mesh> {
    let size = size.into();

    let top = square_mesh(
        context,
        [size.x, size.z],
        [0.0, size.y / 2.0, 0.0],
        Quaternion::axis_rotation(Vector3::RIGHT, 0.0),
    )?;
    let bottom = square_mesh(
        context,
        [size.x, size.z],
        [0.0, -size.y / 2.0, 0.0],
        Quaternion::axis_rotation(Vector3::RIGHT, 180.0),
    )?;

    let left = square_mesh(
        context,
        [size.z, size.y],
        [-size.x / 2.0, 0.0, 0.0],
        Quaternion::axis_rotation(Vector3::FORWARD, 90.0),
    )?;
    let right = square_mesh(
        context,
        [size.z, size.y],
        [size.x / 2.0, 0.0, 0.0],
        Quaternion::axis_rotation(Vector3::FORWARD, -90.0),
    )?;

    let front = square_mesh(
        context,
        [size.x, size.y],
        [0.0, 0.0, -size.z / 2.0],
        Quaternion::axis_rotation(Vector3::RIGHT, -90.0),
    )?;
    let back = square_mesh(
        context,
        [size.x, size.y],
        [0.0, 0.0, size.z / 2.0],
        Quaternion::axis_rotation(Vector3::RIGHT, 90.0),
    )?;

    context.combine_meshes(&[top, bottom, left, right, front, back])
}

fn square_mesh(
    context: &mut Context,
    size: impl Into<Vector2>,
    position: impl Into<Vector3>,
    rotation: Quaternion,
) -> Result<Mesh> {
    let size = size.into();
    let position = position.into();

    let x_pos = size.x / 2.0;
    let x_neg = -size.x / 2.0;
    let z_pos = size.y / 2.0;
    let z_neg = -size.y / 2.0;

    let mut vertices = vec![
        Vector3::new(x_neg, 0.0, z_neg),
        Vector3::new(x_neg, 0.0, z_pos),
        Vector3::new(x_pos, 0.0, z_pos),
        Vector3::new(x_pos, 0.0, z_neg),
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];

    // position and rotate
    for v in vertices.iter_mut() {
        *v = rotation.rotate_vector(*v);
        *v += position;
    }

    let mut mesh = context.create_mesh()?;
    mesh.vertices = vertices;
    mesh.indices = indices;
    mesh.calculate_normals();
    mesh.update();

    Ok(mesh)
}
