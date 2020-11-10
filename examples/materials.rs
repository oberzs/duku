// Oliver Berzs
// https://github.com/oberzs/duku

// Material drawing example

use duku::window::Controller;
use duku::window::Key;
use duku::Camera;
use duku::Color;
use duku::ColorSpace;
use duku::Context;
use duku::Light;
use duku::Mips;
use duku::Result;
use duku::Transform;
use duku::Vector3;

fn main() -> Result<()> {
    let (mut context, window) = Context::builder()
        .no_vsync()
        .build_window(500, 500)
        .title("Twitch Example")
        .resizable()
        .build()?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by([1.0, 3.0, -3.0]);
    camera.transform.look_dir(Vector3::FORWARD);

    let ui = context.create_framebuffer(500, 500);
    let ui_mat = context.build_material().albedo_framebuffer(&ui).build();

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

    let mut max_white_point = 1.0;
    let mut light_brightness = 5.0;

    window.main_loop(move |events| {
        controller.update(&mut camera, events, context.delta_time());

        if events.is_key_typed(Key::Up) {
            light_brightness += 0.1;
        }
        if events.is_key_typed(Key::Down) {
            light_brightness -= 0.1;
        }
        if events.is_key_typed(Key::Right) {
            max_white_point += 0.1;
        }
        if events.is_key_typed(Key::Left) {
            max_white_point -= 0.1;
        }

        // render UI
        context.draw(&ui, None, |target| {
            // move (0, 0) to top left
            target.transform.move_left(250.0);
            target.transform.move_up(250.0);

            target.clear_color = Color::rgba_norm(0.0, 0.0, 0.0, 0.0);

            // left and top margin
            target.transform.move_right(10.0);
            target.transform.move_down(10.0);

            target.text_color = Color::ORANGE;
            target.draw_text(format!("Max White Point: {}", max_white_point), [0.0, 0.0]);
            target.draw_text(
                format!("Light Brightness: {}", light_brightness),
                [0.0, -40.0],
            );
        });

        // render 3D scene
        context.draw_on_window(Some(&camera), |target| {
            target.draw_grid();

            target.lights[0] = Light::main([-1.0, -1.0, 1.0], Color::WHITE, light_brightness);
            target.max_white_point = max_white_point;

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
            target.transform = Transform::scaled(10.0, 0.2, 10.0);
            target.unset_material();
            target.draw_cube();

            // render ui
            target.set_material(&ui_mat);
            target.draw_fullscreen();
        });
    });

    Ok(())
}
