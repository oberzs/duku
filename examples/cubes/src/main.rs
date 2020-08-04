// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh drawing example

use draw_it::controller::Controller;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Mesh;
use draw_it::MeshOptions;
use draw_it::Quaternion;
use draw_it::Result;
use draw_it::Transform;
use draw_it::VSync;
use draw_it::Vector2;
use draw_it::Vector3;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(
        ContextOptions {
            vsync: VSync::Off,
            ..Default::default()
        },
        WindowOptions {
            title: "Draw-it example: Cubes",
            resizable: true,
            width: 720,
            height: 640,
        },
    )?;

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_by([1.0, 3.0, -3.0]);
        cam_t.look_dir(Vector3::FORWARD);
    }

    let mut controller = Controller::orbit([0.0, 0.0, 0.0]);

    context.set_skybox_from_file([
        "examples/cubes/textures/Skybox/top.png",
        "examples/cubes/textures/Skybox/bottom.png",
        "examples/cubes/textures/Skybox/side.png",
        "examples/cubes/textures/Skybox/side.png",
        "examples/cubes/textures/Skybox/side.png",
        "examples/cubes/textures/Skybox/side.png",
    ])?;

    let cube = cube_mesh(&mut context, [1.0, 1.0, 1.0])?;

    let floor_transform = Transform {
        scale: Vector3::new(80.0, 0.2, 80.0),
        position: Vector3::new(0.0, -0.1, 0.0),
        ..Default::default()
    };

    while window.is_open() {
        // update
        context.poll_events(&mut window)?;
        let stats = context.stats();
        controller.update(&mut context.main_camera, &mut window, stats.delta_time);

        // render
        context.draw_ui(|ui| {
            ui.stats_window(stats);
        })?;

        context.draw_on_window(|target| {
            target.set_skybox(true);
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

    // create data
    let vertices = &mut [
        Vector3::new(x_neg, 0.0, z_neg),
        Vector3::new(x_neg, 0.0, z_pos),
        Vector3::new(x_pos, 0.0, z_pos),
        Vector3::new(x_pos, 0.0, z_neg),
    ];
    let indices = &[0, 1, 2, 0, 2, 3];

    // position and rotate
    for v in vertices.iter_mut() {
        *v = rotation.rotate_vector(*v);
        *v += position;
    }

    // generate mesh
    context.create_mesh(MeshOptions {
        vertices,
        indices,
        ..Default::default()
    })
}
