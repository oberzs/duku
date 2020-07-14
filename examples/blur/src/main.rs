// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Gaussian blur example

use tegne::colors;
use tegne::ui;
use tegne::ui::label;
use tegne::CameraType;
use tegne::Controller;
use tegne::Tegne;
use tegne::TegneOptions;
use tegne::Vector3;
use tegne::Window;
use tegne::WindowOptions;

fn main() {
    let (width, height) = (720, 640);
    let blur_scale = 2;
    let mut blur_strength: i32 = 0;
    let blur_strengths = [1, 3, 5, 7];

    let mut window = Window::new(WindowOptions {
        title: "Tegne example: Toon",
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

    let mut controller = Controller::default();

    let hblur_shader = tegne
        .create_shader_from_file("examples/blur/shaders/hblur.shader", Default::default())
        .unwrap();
    let vblur_shader = tegne
        .create_shader_from_file("examples/blur/shaders/vblur.shader", Default::default())
        .unwrap();

    let main_framebuffer = tegne.create_framebuffer(CameraType::Perspective, width, height);
    let hblur_framebuffer = tegne.create_framebuffer(
        CameraType::Orthographic,
        width / blur_scale,
        height / blur_scale,
    );
    let vblur_framebuffer = tegne.create_framebuffer(
        CameraType::Orthographic,
        width / blur_scale,
        height / blur_scale,
    );

    let blur_material = tegne.create_material();

    main_framebuffer.with(|f| {
        let cam_t = &mut f.camera.transform;
        cam_t.move_backward(5.0);
        cam_t.move_up(2.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    });

    window.main_loop(|events, ui| {
        blur_material.with(|m| {
            m.set_arg_1(blur_strengths[blur_strength as usize]);
        });

        main_framebuffer.with(|f| {
            controller.update(&mut f.camera, events);
        });

        ui::stats_window(&ui, &tegne, events);
        ui::Window::new(label!("Blur Settings"))
            .size([1.0, 1.0], ui::Condition::FirstUseEver)
            .always_auto_resize(true)
            .build(&ui, || {
                ui::Slider::new(label!("Strength"), 0..=3).build(&ui, &mut blur_strength);
                ui.text("* This does nothing at this moment");
            });

        tegne.draw_ui(ui);

        tegne.draw(&main_framebuffer, |target| {
            target.set_clear(colors::ORANGE);
            target.draw_cube([-3.0, 1.0, 0.0]);
            target.draw_sphere([-1.0, 1.0, 0.0]);
            target.draw_cube([1.0, 1.0, 0.0]);
            target.draw_sphere([3.0, 1.0, 0.0]);
        });

        tegne.draw(&hblur_framebuffer, |target| {
            target.set_albedo(&main_framebuffer);
            target.set_shader(&hblur_shader);
            target.set_material(&blur_material);
            target.draw_surface();
        });

        tegne.draw(&vblur_framebuffer, |target| {
            target.set_albedo(&hblur_framebuffer);
            target.set_shader(&vblur_shader);
            target.set_material(&blur_material);
            target.draw_surface();
        });

        tegne.draw_on_window(|target| {
            target.blit_framebuffer(&vblur_framebuffer);
        });
    });
}
