// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh drawing example

use draw_it::camera::Controller;
use draw_it::error::Result;
use draw_it::math::Transform;
use draw_it::math::Vector3;
use draw_it::reference::Texture;
use draw_it::shader::SamplerFilter;
use draw_it::shader::SamplerOptions;
use draw_it::ui;
use draw_it::window::Key;
use draw_it::window::Window;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;
use rand::Rng;

struct Cube {
    texture: Texture,
    position: Vector3,
}

fn main() -> Result<()> {
    let (mut width, mut height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Draw-it example: Cubes",
        resizable: true,
        width,
        height,
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            vsync: false,
            ..Default::default()
        },
    )?;

    context.set_skybox_from_file([
        "examples/cubes/textures/Skybox/top.png",
        "examples/cubes/textures/Skybox/bottom.png",
        "examples/cubes/textures/Skybox/side.png",
        "examples/cubes/textures/Skybox/side.png",
        "examples/cubes/textures/Skybox/side.png",
        "examples/cubes/textures/Skybox/side.png",
    ])?;

    let cube_textures = [
        context.create_texture_from_file("examples/cubes/textures/Purple/texture_01.png")?,
        context.create_texture_from_file("examples/cubes/textures/Orange/texture_05.png")?,
        context.create_texture_from_file("examples/cubes/textures/Green/texture_13.png")?,
    ];
    let floor_texture =
        context.create_texture_from_file("examples/cubes/textures/Light/texture_06.png")?;

    let floor_transform = Transform {
        scale: Vector3::new(80.0, 0.2, 80.0),
        position: Vector3::new(0.0, -0.1, 0.0),
        ..Default::default()
    };

    let mut rng = rand::thread_rng();
    let cubes = (0..20)
        .map(|i| {
            let t = rng.gen_range(0, cube_textures.len());
            let y = rng.gen_range(0, 3);
            let z = rng.gen_range(-10, 10);
            Cube {
                texture: cube_textures[t].clone(),
                position: Vector3::new(10.0 - i as f32 + 0.5, y as f32 + 0.5, z as f32 + 0.5),
            }
        })
        .collect::<Vec<_>>();

    {
        let cam_t = &mut context.main_camera.transform;
        cam_t.move_by([0.0, 1.0, -15.0]);
        cam_t.look_in_dir(Vector3::forward(), Vector3::up());
    }

    let mut controller = Controller::default();

    let mut paused = false;

    window.main_loop(|events, ui| {
        if events.is_key_typed(Key::P) {
            paused = !paused;
            events.set_title(if paused {
                "Draw-it example: Cubes (paused)"
            } else {
                "Draw-it example: Cubes"
            });
        }

        if !paused {
            controller.update(&mut context.main_camera, events);

            let wireframes = events.is_key_pressed(Key::E);

            if let Some((new_width, new_height)) = events.resized() {
                context.resize(new_width, new_height)?;
                width = new_width;
                height = new_height;
            }

            ui::stats_window(&ui, &context, events);

            context.draw_ui(ui)?;
            context.draw_on_window(|target| {
                target.set_wireframes(wireframes);
                target.set_skybox(true);

                // draw floor
                target.set_sampler(SamplerOptions {
                    filter: SamplerFilter::Nearest,
                    ..Default::default()
                });
                target.set_albedo(&floor_texture);
                target.draw_cube(floor_transform);
                target.set_sampler(Default::default());

                // draw cubes
                for cube in &cubes {
                    target.set_albedo(&cube.texture);
                    target.draw_cube(cube.position);
                }
            })?;
        }

        Ok(())
    });

    Ok(())
}
