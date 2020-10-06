// Oliver Berzs
// https://github.com/oberzs/draw-it

// Mesh drawing example

use draw_it::window::Controller;
use draw_it::Camera;
use draw_it::Color;
use draw_it::Context;
use draw_it::CubemapSides;
use draw_it::Handle;
use draw_it::Light;
use draw_it::Mesh;
use draw_it::Quaternion;
use draw_it::Result;
use draw_it::Vector2;
use draw_it::Vector3;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .no_vsync()
        .build_window(720, 640)
        .title("Draw-it example: Cubes")
        .resizable()
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by((1.0, 3.0, -3.0));
    camera.transform.look_dir(Vector3::FORWARD);

    let mut controller = Controller::orbit((0.0, 0.0, 0.0));

    let texture = context.create_texture_png("examples/textures/Orange/texture_01.png")?;

    let material = context
        .build_material()
        .phong_color((255, 255, 255))
        .phong_texture(&texture)
        .build();

    context.set_skybox_png(CubemapSides {
        top: "examples/textures/Skybox/glacier_up.png",
        bottom: "examples/textures/Skybox/glacier_down.png",
        front: "examples/textures/Skybox/glacier_front.png",
        back: "examples/textures/Skybox/glacier_back.png",
        left: "examples/textures/Skybox/glacier_left.png",
        right: "examples/textures/Skybox/glacier_right.png",
    })?;

    let cube = cube_mesh(&mut context, (1.0, 1.0, 1.0));

    let main_light = Light::directional((-0.4, -1.0, -1.0), Color::WHITE, true);

    while window.is_open() {
        // update
        context.poll_events(&mut window);

        let delta_time = context.delta_time();
        controller.update(&mut camera, &mut window, delta_time);

        // render
        let stats = context.stats();
        let fps = context.fps();
        context.draw_ui(|ui| {
            ui.stats_window(stats, fps, delta_time);
        })?;

        context.draw_on_window(Some(&camera), |target| {
            target.skybox = true;
            target.lights[0] = main_light;

            target.draw_grid();

            // render custom mesh
            target.transform.move_by((-2.0, 1.0, 0.0));
            target.draw_mesh(&cube);

            // render builtin meshes
            target.transform.move_right(2.0);
            target.draw_sphere();
            target.material = Some(&material);
            target.transform.move_right(2.0);
            target.draw_cube();
        });
    }

    Ok(())
}

fn cube_mesh(context: &mut Context, size: impl Into<Vector3>) -> Handle<Mesh> {
    let size = size.into();

    let top = square_mesh(
        context,
        (size.x, size.z),
        (0.0, size.y / 2.0, 0.0),
        Quaternion::axis_rotation(Vector3::RIGHT, 0.0),
    );
    let bottom = square_mesh(
        context,
        (size.x, size.z),
        (0.0, -size.y / 2.0, 0.0),
        Quaternion::axis_rotation(Vector3::RIGHT, 180.0),
    );

    let left = square_mesh(
        context,
        (size.z, size.y),
        (-size.x / 2.0, 0.0, 0.0),
        Quaternion::axis_rotation(Vector3::FORWARD, 90.0),
    );
    let right = square_mesh(
        context,
        (size.z, size.y),
        (size.x / 2.0, 0.0, 0.0),
        Quaternion::axis_rotation(Vector3::FORWARD, -90.0),
    );

    let front = square_mesh(
        context,
        (size.x, size.y),
        (0.0, 0.0, -size.z / 2.0),
        Quaternion::axis_rotation(Vector3::RIGHT, -90.0),
    );
    let back = square_mesh(
        context,
        (size.x, size.y),
        (0.0, 0.0, size.z / 2.0),
        Quaternion::axis_rotation(Vector3::RIGHT, 90.0),
    );

    context.combine_meshes(&[top, bottom, left, right, front, back])
}

fn square_mesh(
    context: &mut Context,
    size: impl Into<Vector2>,
    position: impl Into<Vector3>,
    rotation: Quaternion,
) -> Handle<Mesh> {
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

    context
        .build_mesh()
        .vertices(vertices)
        .indices(indices)
        .calculated_normals()
        .build()
}
