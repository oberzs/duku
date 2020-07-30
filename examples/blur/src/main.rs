// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// Gaussian blur example

use draw_it::colors;
use draw_it::controller::Controller;
use draw_it::ui;
use draw_it::ui::label;
use draw_it::window::WindowOptions;
use draw_it::CameraType;
use draw_it::Context;
use draw_it::Result;
use draw_it::Vector3;

fn main() -> Result<()> {
    let (width, height) = (720, 640);
    let blur_scale = 2;
    let mut blur_strength: i32 = 0;
    let blur_strengths = [1, 3, 5, 7];

    let (mut context, mut window) = Context::with_window(
        Default::default(),
        WindowOptions {
            title: "Draw-it example: Blur",
            width,
            height,
            ..Default::default()
        },
    )?;

    let mut controller = Controller::default();

    let hblur_shader = context
        .create_shader_from_file("examples/blur/shaders/hblur.shader", Default::default())?;
    let vblur_shader = context
        .create_shader_from_file("examples/blur/shaders/vblur.shader", Default::default())?;

    let main_framebuffer = context.create_framebuffer(CameraType::Perspective, width, height)?;
    let hblur_framebuffer = context.create_framebuffer(
        CameraType::Orthographic,
        width / blur_scale,
        height / blur_scale,
    )?;
    let vblur_framebuffer = context.create_framebuffer(
        CameraType::Orthographic,
        width / blur_scale,
        height / blur_scale,
    )?;

    let blur_material = context.create_material()?;

    main_framebuffer.with(|f| {
        let cam_t = &mut f.camera.transform;
        cam_t.move_backward(5.0);
        cam_t.move_up(2.0);
        cam_t.look_at([0.0, 0.0, 0.0], Vector3::up());
    });

    while window.is_open() {
        context.poll_events(&mut window)?;

        blur_material.with(|m| {
            m.set_arg_1(blur_strengths[blur_strength as usize]);
        });

        main_framebuffer.with(|f| {
            controller.update(&mut f.camera, &mut window);
        });

        let stats = context.render_stats();
        context.draw_ui(|ui| {
            ui::stats_window(&ui, stats, &window);
            ui::Window::new(label!("Blur Settings"))
                .size([1.0, 1.0], ui::Condition::FirstUseEver)
                .always_auto_resize(true)
                .build(&ui, || {
                    ui::Slider::new(label!("Strength"), 0..=3).build(&ui, &mut blur_strength);
                    ui.text("* This does nothing at this moment");
                });
        })?;

        context.draw(&main_framebuffer, |target| {
            target.set_clear(colors::ORANGE);
            target.draw_cube([-3.0, 1.0, 0.0]);
            target.draw_sphere([-1.0, 1.0, 0.0]);
            target.draw_cube([1.0, 1.0, 0.0]);
            target.draw_sphere([3.0, 1.0, 0.0]);
        })?;

        context.draw(&hblur_framebuffer, |target| {
            target.set_albedo(&main_framebuffer);
            target.set_shader(&hblur_shader);
            target.set_material(&blur_material);
            target.draw_surface();
        })?;

        context.draw(&vblur_framebuffer, |target| {
            target.set_albedo(&hblur_framebuffer);
            target.set_shader(&vblur_shader);
            target.set_material(&blur_material);
            target.draw_surface();
        })?;

        context.draw_on_window(|target| {
            target.blit_framebuffer(&vblur_framebuffer);
        })?;
    }

    Ok(())
}
