// Oliver Berzs
// https://github.com/oberzs/draw-it

// Material drawing example

use draw_it::window::Controller;
use draw_it::Camera;
use draw_it::Color;
use draw_it::Context;
use draw_it::CubemapSides;
use draw_it::Light;
use draw_it::Result;
use draw_it::Transform;
use draw_it::Vector3;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder()
        .no_vsync()
        .build_window(720, 640)
        .title("Draw-it example: Materials")
        .resizable()
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by((1.0, 3.0, -3.0));
    camera.transform.look_dir(Vector3::FORWARD);

    let mut controller = Controller::orbit((0.0, 0.0, 0.0));

    let orange_tex = context.create_texture_png("examples/textures/Orange/texture_01.png")?;
    let painted_albedo_tex =
        context.create_texture_png("examples/textures/painted/painted-albedo.png")?;
    let painted_roughness_tex =
        context.create_texture_png_linear("examples/textures/painted/painted-roughness.png")?;
    let painted_metalness_tex =
        context.create_texture_png_linear("examples/textures/painted/painted-metalness.png")?;

    let orange_mat = context
        .build_material()
        .albedo_color((255, 255, 255))
        .albedo_texture(&orange_tex)
        .build();
    let painted_mat = context
        .build_material()
        .albedo_color((255, 255, 255))
        .albedo_texture(&painted_albedo_tex)
        .metalness(1.0)
        .metalness_texture(&painted_metalness_tex)
        .roughness(1.0)
        .roughness_texture(&painted_roughness_tex)
        .build();

    context.set_skybox_png(CubemapSides {
        top: "examples/textures/Skybox/glacier_up.png",
        bottom: "examples/textures/Skybox/glacier_down.png",
        front: "examples/textures/Skybox/glacier_front.png",
        back: "examples/textures/Skybox/glacier_back.png",
        left: "examples/textures/Skybox/glacier_left.png",
        right: "examples/textures/Skybox/glacier_right.png",
    })?;

    while window.is_open() {
        // update
        context.poll_events(&mut window);

        let delta_time = context.delta_time();
        controller.update(&mut camera, &mut window, delta_time);

        context.draw_on_window(Some(&camera), |target| {
            target.skybox = true;
            target.lights[0] = Light::main((-0.4, -1.0, -1.0), Color::WHITE, 10.0);
            target.lights[1] = Light::point((-3.0, 0.0, 1.0), Color::BLUE, 1.0);
            target.lights[2] = Light::point((3.0, 0.0, 1.0), Color::GREEN, 1.0);

            target.draw_grid();

            // render meshes
            target.transform.move_by((-2.0, 1.0, 0.0));
            target.material = Some(&painted_mat);
            target.draw_sphere_uv();

            target.transform.move_right(2.0);
            target.material = None;
            target.draw_sphere_ico();

            target.transform.move_right(2.0);
            target.material = Some(&orange_mat);
            target.draw_cube();

            // render floor
            target.transform = Transform::scaled(10.0, 0.2, 10.0);
            target.material = None;
            target.draw_cube();
        });
    }

    Ok(())
}
