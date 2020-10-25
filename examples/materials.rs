// Oliver Berzs
// https://github.com/oberzs/draw-it

// Material drawing example

use draw_it::window::Controller;
use draw_it::Camera;
use draw_it::Context;
use draw_it::CubemapSides;
use draw_it::Result;
use draw_it::Transform;
use draw_it::Vector3;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .no_vsync()
        .build_window(720, 640)
        .title("Draw-it example: Materials")
        .resizable()
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by((1.0, 3.0, -3.0));
    camera.transform.look_dir(Vector3::FORWARD);

    let mut controller = Controller::orbit((0.0, 0.0, 0.0));

    let light_tex = context.create_texture_png("examples/textures/prototype/light.png")?;
    let purple_tex = context.create_texture_png("examples/textures/prototype/purple.png")?;

    let light_mat = context
        .build_material_pbr()
        .albedo_texture(&light_tex)
        .metalness(1.0)
        .roughness(0.5)
        .build();
    let purple_mat = context
        .build_material_pbr()
        .albedo_texture(&purple_tex)
        .build();

    context.set_skybox_png(CubemapSides {
        top: "examples/textures/skybox/top.png",
        bottom: "examples/textures/skybox/bottom.png",
        front: "examples/textures/skybox/front.png",
        back: "examples/textures/skybox/back.png",
        left: "examples/textures/skybox/left.png",
        right: "examples/textures/skybox/right.png",
    })?;

    window.main_loop(move |events| {
        controller.update(&mut camera, events, context.delta_time());

        context.draw_on_window(Some(&camera), |target| {
            target.skybox = true;

            target.draw_grid();

            // render meshes
            target.transform.move_by((-2.0, 1.0, 0.0));
            target.material = Some(&light_mat);
            target.draw_sphere_uv();

            target.transform.move_right(2.0);
            target.material = None;
            target.draw_sphere_ico();

            target.transform.move_right(2.0);
            target.material = Some(&purple_mat);
            target.draw_cube();

            // render floor
            target.transform = Transform::scaled(10.0, 0.2, 10.0);
            target.material = None;
            target.draw_cube();
        });
    });

    Ok(())
}
