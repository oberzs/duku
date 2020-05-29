// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// UI struct with framebuffer that displays framerate

use tegne::CameraType;
use tegne::Events;
use tegne::Framebuffer;
use tegne::Id;
use tegne::Material;
use tegne::MaterialOptions;
use tegne::Tegne;
use tegne::Transform;
use tegne::Vector3;

pub struct Ui {
    framebuffer: Id<Framebuffer>,
    material: Id<Material>,
    transform: Transform,
}

impl Ui {
    pub fn new(tegne: &Tegne, width: u32, height: u32) -> Self {
        let framebuffer = tegne.create_framebuffer(CameraType::Orthographic, width, height);
        let material = tegne.create_material(MaterialOptions {
            albedo_tint: Vector3::new(1.0, 0.0, 1.0),
            font_width: 0.5,
            font_edge: 0.15,
            ..Default::default()
        });
        let scale = 32.0;
        let transform = Transform {
            position: Vector3::new(
                -(width as f32) / 2.0 + 5.0,
                ((height as f32) / 2.0) - scale,
                1.0,
            ),
            scale: Vector3::new(scale, scale, scale),
            ..Default::default()
        };

        Self {
            framebuffer,
            material,
            transform,
        }
    }

    pub fn draw_ui(&self, tegne: &mut Tegne, events: &Events) {
        tegne.draw(&self.framebuffer, |target| {
            target.set_clear([0.0, 0.0, 0.0, 0.0]);
            target.set_material(&self.material);
            target.draw_text(format!("fps: {}", events.fps()), self.transform);
        });
    }

    pub fn framebuffer(&self) -> &Id<Framebuffer> {
        &self.framebuffer
    }
}
