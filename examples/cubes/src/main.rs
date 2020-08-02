// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Mesh drawing example

use draw_it::controller::Controller;
use draw_it::window::Key;
use draw_it::window::WindowOptions;
use draw_it::Context;
use draw_it::ContextOptions;
use draw_it::Result;
use draw_it::SamplerFilter;
use draw_it::SamplerOptions;
use draw_it::Texture;
use draw_it::Transform;
use draw_it::VSync;
use draw_it::Vector3;
use rand::Rng;

struct Cube {
    texture: Texture,
    position: Vector3,
}

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
        cam_t.look_dir(Vector3::FORWARD);
    }

    let mut controller = Controller::default();

    let mut paused = false;

    while window.is_open() {
        context.poll_events(&mut window)?;

        if window.is_key_typed(Key::P) {
            paused = !paused;
            window.set_title(if paused {
                "Draw-it example: Cubes (paused)"
            } else {
                "Draw-it example: Cubes"
            });
        }

        if !paused {
            let stats = context.stats();

            controller.update(&mut context.main_camera, &mut window, stats.delta_time);

            let wireframes = window.is_key_pressed(Key::E);

            context.draw_ui(|ui| {
                ui.stats_window(stats);
            })?;

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
    }

    Ok(())
}
