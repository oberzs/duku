// Oliver Berzs
// https://github.com/oberzs/duku

// Material drawing example

use duku::window::Controller;
use duku::Camera;
use duku::Color;
use duku::ColorSpace;
use duku::CubemapSides;
use duku::Duku;
use duku::Light;
use duku::Mips;
use duku::Result;
use duku::Vector3;

fn main() -> Result<()> {
    let (mut duku, window) = Duku::builder()
        .no_vsync()
        .build_window(500, 500)
        .title("Duku example: Materials")
        .resizable()
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by([1.0, 3.0, -3.0]);

    let mut controller = Controller::orbit([0.0, 0.0, 0.0]);

    let skybox = duku.create_cubemap_png(
        ColorSpace::Srgb,
        CubemapSides {
            top: "examples/textures/skybox/top.png",
            bottom: "examples/textures/skybox/bottom.png",
            front: "examples/textures/skybox/front.png",
            back: "examples/textures/skybox/back.png",
            left: "examples/textures/skybox/left.png",
            right: "examples/textures/skybox/right.png",
        },
    )?;

    let light_tex = duku.create_texture_png(
        "examples/textures/prototype/light.png",
        ColorSpace::Srgb,
        Mips::Log2,
    )?;
    let purple_tex = duku.create_texture_png(
        "examples/textures/prototype/purple.png",
        ColorSpace::Srgb,
        Mips::Log2,
    )?;

    let mut light_mat = duku.create_material_pbr()?;
    light_mat.albedo_texture(light_tex);
    light_mat.metalness(1.0);
    light_mat.roughness(0.5);
    let mut purple_mat = duku.create_material_pbr()?;
    purple_mat.albedo_texture(purple_tex);

    window.main_loop(move |events| {
        controller.update(&mut camera, events, duku.delta_time());

        duku.draw_on_window(Some(&camera), |target| {
            target.set_skybox(&skybox);
            target.lights[0] = Light::main([-1.0, -1.0, 1.0], Color::WHITE, 5.0);

            target.draw_grid();

            // render meshes
            target.transform.move_by([-2.0, 1.0, 0.0]);
            target.set_material(&light_mat);
            target.draw_sphere_uv();

            target.transform.move_right(2.0);
            target.unset_material();
            target.draw_sphere_ico();

            target.transform.move_right(2.0);
            target.set_material(&purple_mat);
            target.draw_cube();

            // render floor
            target.transform.position = Vector3::default();
            target.transform.scale = Vector3::new(10.0, 0.2, 10.0);
            target.unset_material();
            target.draw_cube();
        });
    });

    Ok(())
}
