// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Mesh drawing example

mod cube;
mod floor;

use rand::Rng;
use tegne::ui;
use tegne::Controller;
use tegne::Key;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

use cube::Cube;
use floor::Floor;

fn main() {
    let (mut width, mut height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Cubes",
        resizable: true,
        width,
        height,
    });
    let mut tegne = Tegne::from_window(
        &mut window,
        TegneOptions {
            anisotropy: 16.0,
            msaa: 4,
            vsync: false,
            ..Default::default()
        },
    );

    let cube_textures = [
        tegne
            .create_texture_from_file("examples/cubes/textures/Purple/texture_01.png")
            .unwrap(),
        tegne
            .create_texture_from_file("examples/cubes/textures/Orange/texture_05.png")
            .unwrap(),
        tegne
            .create_texture_from_file("examples/cubes/textures/Green/texture_13.png")
            .unwrap(),
    ];

    let floor = Floor::new(&mut tegne);

    let mut rng = rand::thread_rng();
    let cubes = (0..1)
        .map(|i| {
            let t = rng.gen_range(0, cube_textures.len());
            let y = rng.gen_range(0, 3);
            let z = rng.gen_range(-10, 10);
            Cube::new(
                &mut tegne,
                &cube_textures[t],
                [10.0 - i as f32, y as f32, z as f32],
            )
        })
        .collect::<Vec<_>>();

    {
        let cam_t = &mut tegne.main_camera.transform;
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
            controller.update(&mut tegne.main_camera, events);

            let wireframes = events.is_key_pressed(Key::E);

            if let Some((new_width, new_height)) = events.resized() {
                tegne.resize(new_width, new_height);
                width = new_width;
                height = new_height;
            }

            ui::stats_window(&ui, &tegne, events);

            tegne.draw_ui(ui);
            tegne.draw_on_window(|target| {
                target.set_wireframes(wireframes);
                floor.draw(target);
                for cube in &cubes {
                    cube.draw(target);
                }
                target.draw_debug_sphere([0.0, 5.0, 0.0]);
                target.draw_debug_cube([0.0, 3.0, 0.0]);
            });
        }
    });
}
