// Oliver Berzs
// https://github.com/oberzs/duku

use duku::Duku;
use duku::Handle;
use duku::Result;
use duku::Texture;
use duku::Vec2;

struct Animation {
    frames: Vec<Frame>,
    current: f32,
    speed: f32,
}

#[derive(Copy, Clone)]
struct Frame {
    offset: Vec2,
    size: Vec2,
}

fn main() -> Result<()> {
    // create duku context and window
    let (mut duku, window) = Duku::windowed(500, 500)?;

    let run = duku.create_texture_png("examples/textures/run.png", None)?;

    let mut animation = Animation::new(&run, 10.0, 8);

    // start window loop
    window.while_open(move |_| {
        animation.update(duku.delta_time());

        // start drawing on window
        duku.draw(None, |t| {
            let Frame { offset, size } = animation.get();
            t.texture_part(&run, [0.0, 0.0], size, offset, size);
        });
    });

    Ok(())
}

impl Animation {
    fn new(texture: &Handle<Texture>, speed: f32, frame_count: u32) -> Self {
        let width = texture.width() / frame_count;
        let height = texture.height();

        let frames: Vec<_> = (0..frame_count)
            .map(|i| Frame {
                offset: Vec2::new((i * width) as f32, 0.0),
                size: Vec2::new(width as f32, height as f32),
            })
            .collect();

        Self {
            current: 0.0,
            speed,
            frames,
        }
    }

    fn update(&mut self, dt: f32) {
        self.current += self.speed * dt;
    }

    fn get(&self) -> Frame {
        self.frames[self.current as usize % self.frames.len()]
    }
}
