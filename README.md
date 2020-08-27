<h1 align="center">Draw it 🎨</h1>

<div align="center">
  <strong>Rendering library written in Rust using Vulkan</strong>
</div>

<br />

<div align="center">
  <!-- Version -->
  <span>
    <img src="https://img.shields.io/badge/version-Work%20In%20Progress-yellow?style=flat-square" alt="Version" />
  </span>
  <!-- Build status -->
  <span>
    <img src="https://img.shields.io/github/workflow/status/OllieBerzs/draw-it/Full%20Build?style=flat-square" alt="Build Status" />
  </span>
  <!-- Rust Version -->
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/rust-1.45.2-orange?style=flat-square" alt="Rust Version" />
  </a>
  <!-- License -->
  <a href="https://github.com/OllieBerzs/draw-it/blob/develop/LICENSE">
    <img src="https://img.shields.io/github/license/OllieBerzs/draw-it?style=flat-square" alt="License" />
  </a>
</div>

<div align="center">
  <a href="#features">Features</a> •
  <a href="#support">Support</a> •
  <a href="#simple-example">Simple Example</a> •
  <a href="#credits">Credits</a>
</div>

## Features

- **easy to use:** written with a goal to be as easy to use as possible
- **2D/3D:** can be used for both
- **shaders:** write custom shaders in a modified GLSL
- **SDF fonts:** signed distance field font support
- **shadows:** uses PCF shadow maps
- **windowing agnostic:** can be used with any OS window
- **hot-reload:** shaders can be reloaded while the app is running

## Support

- **Windows:** 🟢
- **Linux X11:** 🟢
- **Linux Wayland:** 🔴 planned, not supported
- **MacOS:** 🔴 not tested, help wanted

## Simple Example

```rust
use draw_it::Color;
use draw_it::window::Window;
use draw_it::Camera;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::with_window(Default::default(), Default::default())?;

    let mut camera = Camera::perspective_autosized(90);
    camera.transform.move_by([2.0, 1.5, 2.0]);
    camera.transform.look_at([0.0, 0.0, 0.0]);

    while window.is_open() {
        context.poll_events(&mut window);
        context.draw_on_window(Some(&camera), |target| {
            target.clear = Color::SKY_BLUE;
            target.draw_cube([0.0, 0.0, 0.0]);
        });
    }

    Ok(())
}
```

Want more? Check out these other [examples](https://github.com/OllieBerzs/draw-it/tree/develop/examples).

## Credits

This library uses these open source projects:

- [Vulkan SDK](https://vulkan.lunarg.com/) - Vulkan API support
- [Serde](https://github.com/serde-rs/serde) - Deserialize assets
- [Bincode](https://github.com/servo/bincode) - Serde helper for binary

Assets

- [Kenney](https://www.kenney.nl/assets) - Assets for examples and fonts
- [Skybox](https://www.moddb.com/addons/cc0-skybox-pack-1) - Skybox for Cube example

Optional projects for extra functionality:

- [Glfw](https://github.com/PistonDevelopers/glfw-rs) - OS Windowing
- [Imgui](https://github.com/Gekkio/imgui-rs) - Easy UI
