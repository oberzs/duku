// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Mesh drawing example

mod cube;
mod floor;

use rand::Rng;
use std::time::Instant;
use tegne::ui;
use tegne::ui::im_str;
use tegne::CameraType;
use tegne::Controller;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

use cube::Cube;
use floor::Floor;

fn main() {
    pretty_env_logger::init();

    let (width, height) = (720, 640);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Cubes",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(
        &mut window,
        TegneOptions {
            anisotropy: 16.0,
            msaa: 4,
            ..Default::default()
        },
    );

    let start_time = Instant::now();

    let floor = Floor::new(&tegne);

    let mut rng = rand::thread_rng();
    let cubes = (0..20)
        .map(|i| {
            let y = rng.gen_range(0.0, 3.0);
            let z = rng.gen_range(-10.0, 10.0);
            let size = rng.gen_range(0.5, 1.0);
            Cube::new(&tegne, [10.0 - i as f32, y, z], size)
        })
        .collect::<Vec<_>>();

    let load_time = start_time.elapsed().as_secs_f32();

    let ui_frame = tegne.create_framebuffer(CameraType::Orthographic, width, height);

    {
        let cam_t = &mut tegne.main_camera.transform;
        cam_t.move_by([0.0, 5.0, -10.0]);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    let mut controller = Controller::default();

    window.main_loop(|events, ui| {
        controller.update(&mut tegne.main_camera, events);

        ui::Window::new(im_str!("Stats"))
            .position([0.0, 0.0], ui::Condition::FirstUseEver)
            .size([100.0, 100.0], ui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .resizable(false)
            .build(&ui, || {
                ui.text(format!("Load time: {}s", load_time));
                ui.text(format!("Fps: {}", events.fps()));
            });
        let ui_data = ui.render();

        tegne.draw(&ui_frame, |target| {
            target.set_clear([0, 0, 0, 0]);
            target.draw_ui(ui_data);
        });

        tegne.draw_on_window(|target| {
            floor.draw(target);
            for cube in &cubes {
                cube.draw(target);
            }
            target.blit_framebuffer(&ui_frame);
        });
    });
}
