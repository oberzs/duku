// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Mesh drawing example

use rand::Rng;
use tegne::ui;
use tegne::Context;
use tegne::ContextOptions;
use tegne::Controller;
use tegne::Key;
use tegne::SamplerFilter;
use tegne::SamplerOptions;
use tegne::Texture;
use tegne::Transform;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

struct Cube {
    texture: Texture,
    position: Vector3,
}

fn main() {
    let (mut width, mut height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Cubes",
        resizable: true,
        width,
        height,
    });
    let mut context = Context::from_window(
        &mut window,
        ContextOptions {
            anisotropy: 16.0,
            msaa: 4,
            vsync: false,
            ..Default::default()
        },
    );

    let cube_textures = [
        context
            .create_texture_from_file("examples/cubes/textures/Purple/texture_01.png")
            .unwrap(),
        context
            .create_texture_from_file("examples/cubes/textures/Orange/texture_05.png")
            .unwrap(),
        context
            .create_texture_from_file("examples/cubes/textures/Green/texture_13.png")
            .unwrap(),
    ];
    let floor_texture = context
        .create_texture_from_file("examples/cubes/textures/Light/texture_06.png")
        .unwrap();

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
                "Tegne example: Cubes (paused)"
            } else {
                "Tegne example: Cubes"
            });
        }

        if !paused {
            controller.update(&mut context.main_camera, events);

            let wireframes = events.is_key_pressed(Key::E);

            if let Some((new_width, new_height)) = events.resized() {
                context.resize(new_width, new_height);
                width = new_width;
                height = new_height;
            }

            ui::stats_window(&ui, &context, events);

            context.draw_ui(ui);
            context.draw_on_window(|target| {
                target.set_wireframes(wireframes);

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
            });
        }
    });
}
