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
    <img src="https://img.shields.io/github/workflow/status/oberzs/draw-it/Full%20Build?style=flat-square" alt="Build Status" />
  </span>
  <!-- Rust Version -->
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/rust-1.46.0-orange?style=flat-square" alt="Rust Version" />
  </a>
  <!-- License -->
  <a href="https://github.com/oberzs/draw-it/blob/develop/LICENSE">
    <img src="https://img.shields.io/github/license/oberzs/draw-it?style=flat-square" alt="License" />
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
- **shadows:** uses PCF shadow maps
- **windowing agnostic:** can be used with any OS window
- **hot-reload:** shaders can be reloaded while the app is running
- **minimal dependencies:** all Rust dependencies are optional

## Support

- [x] **Windows**
- [x] **Linux X11**
- [ ] **Linux Wayland** - planned, not supported
- [ ] **MacOS** - not tested, help wanted

## Simple Example

```rust
use draw_it::Color;
use draw_it::Camera;
use draw_it::Context;
use draw_it::Result;

fn main() -> Result<()> {
    let (mut context, mut window) = Context::builder().build_window(500, 500).build()?;

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

Want more? Check out these other [examples](https://github.com/oberzs/draw-it/tree/develop/examples).

## Credits

This library uses these open source projects:

- [Vulkan SDK](https://vulkan.lunarg.com/) - Vulkan API support

Assets

- [Kenney](https://www.kenney.nl/assets) - Assets for examples and fonts
- [Fira Mono](https://fonts.google.com/specimen/Fira+Mono?query=fira) - Builtin font
- [Skybox](https://www.moddb.com/addons/cc0-skybox-pack-1) - Skybox for Cube example

Optional projects for extra functionality:

- [Png](https://github.com/image-rs/image-png) - Png image loading
- [Shaderc](https://github.com/google/shaderc-rs) - Glsl shader loading
- [Glfw](https://github.com/PistonDevelopers/glfw-rs) - OS Windowing
- [Imgui](https://github.com/Gekkio/imgui-rs) - Easy UI
