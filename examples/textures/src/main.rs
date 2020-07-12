// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// example to draw textures to the window

use tegne::CameraType;
use tegne::Key;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Transform;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

const TEXTURE_SIZE: u32 = 512;

fn main() {
    let (width, height) = (TEXTURE_SIZE * 2, TEXTURE_SIZE);

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Textures",
        width,
        height,
        ..Default::default()
    });
    let mut tegne = Tegne::from_window(
        &mut window,
        TegneOptions {
            camera: CameraType::Orthographic,
            ..Default::default()
        },
    );

    let texture_1 = tegne
        .create_texture_from_file("examples/textures/textures/ShaderBall_A_CYCLO.png")
        .unwrap();
    let texture_2 = tegne
        .create_texture_from_file("examples/textures/textures/ShaderBall_A_REPERE.png")
        .unwrap();

    let mut transform_1 = Transform::default();
    transform_1.position = Vector3::new(-(TEXTURE_SIZE as f32), -100.0, 0.0);
    transform_1.scale = Vector3::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32, 1.0);

    let mut transform_2 = Transform::default();
    transform_2.position = Vector3::new(0.0, 0.0, 0.0);
    transform_2.scale = Vector3::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32, 1.0);

    window.main_loop(|events, _| {
        if events.is_key_pressed(Key::Down) {
            transform_1.position.y -= 1.0;
        }
        if events.is_key_pressed(Key::Up) {
            transform_1.position.y += 1.0;
        }

        tegne.draw_on_window(|target| {
            target.draw_texture(&texture_1, transform_1);
            target.draw_texture(&texture_2, transform_2);
        });
    });
}
