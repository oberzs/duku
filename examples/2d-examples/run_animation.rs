// Oliver Berzs
// https://github.com/oberzs/duku

// This example splits a sprite sheet texture into
// parts and draws it as an animation.

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
    let (mut duku, window) = Duku::windowed(256, 256);

    // load sprite sheet into texture
    let run = duku.create_texture_png("examples/textures/run.png", None)?;

    // split texture into animation parts
    let mut animation = Animation::new(&run, 10.0, 8);

    // start window loop
    window.while_open(move |_| {
        // update the animation with the delta time
        animation.update(duku.delta_time());

        // start drawing on window
        duku.begin();
        duku.draw(None, |t| {
            let Frame { offset, size } = animation.get();
            t.texture_part(&run, [0.0, 0.0], size, offset, size);
        });
        duku.end();
    });

    Ok(())
}

impl Animation {
    fn new(texture: &Handle<Texture>, speed: f32, frame_count: u32) -> Self {
        let (width, height) = {
            let tex = texture.read();
            (tex.width() / frame_count, tex.height())
        };

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
