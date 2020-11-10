// Oliver Berzs
// https://github.com/oberzs/duku

// Material drawing example

use duku::window::Controller;
use duku::Camera;
use duku::ColorSpace;
use duku::Context;
use duku::Mips;
use duku::Result;
use duku::Transform;
use duku::Vector3;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .no_vsync()
        .build_window(720, 640)
        .title("Duku example: Materials")
        .resizable()
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by([1.0, 3.0, -3.0]);
    camera.transform.look_dir(Vector3::FORWARD);

    let mut controller = Controller::orbit([0.0, 0.0, 0.0]);

    let light_tex = context.create_texture_png(
        "examples/textures/prototype/light.png",
        ColorSpace::Srgb,
        Mips::Log2,
    )?;
    let purple_tex = context.create_texture_png(
        "examples/textures/prototype/purple.png",
        ColorSpace::Srgb,
        Mips::Log2,
    )?;

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

    window.main_loop(move |events| {
        controller.update(&mut camera, events, context.delta_time());

        context.draw_on_window(Some(&camera), |target| {
            target.draw_grid();

            // render meshes
            target.transform.move_by([-2.0, 1.0, 0.0]);
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
