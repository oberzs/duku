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
    <img src="https://img.shields.io/github/workflow/status/OllieBerzs/draw-it/Build?style=flat-square" alt="Build Status" />
  </span>
  <!-- Rust Version -->
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/rust-1.46.0--nightly-orange?style=flat-square" alt="Rust Version" />
  </a>
  <!-- License -->
  <a href="https://github.com/OllieBerzs/draw-it/blob/develop/LICENSE">
    <img src="https://img.shields.io/github/license/OllieBerzs/draw-it?style=flat-square" alt="License" />
  </a>
</div>

<div align="center">
  <a href="#features">Features</a> •
  <a href="#simple-example">Simple Example</a> •
  <a href="#credits">Credits</a>
</div>

## Features

- **easy to use:** written with a goal to be as easy to use as possible
- **2D/3D:** can be used for both
- **shaders:** write custom shaders in a modified GLSL
- **SDF fonts:** signed distance field font support
- **shadows**: uses variance shadow maps
- **windowing agnostic:** can be used with any OS window

## Simple Example

```rust
use draw_it::color::colors;
use draw_it::math::Vector3;
use draw_it::window::Window;
use draw_it::Context;

fn main() {
    let mut window = Window::new(Default::default());
    let mut context = Context::from_window(&mut window, Default::default());

    {
        let camera = &mut context.main_camera.transform;
        camera.move_by([2.0, 1.5, 2.0]);
        camera.look_at([0.0, 0.0, 0.0], Vector3::up());
    }

    window.main_loop(|_, _| {
        context.draw_on_window(|target| {
            target.set_clear(colors::SKY_BLUE);
            target.draw_cube([0.0, 0.0, 0.0]);
        });
    });
}
```

Want more? Check out these other [examples](https://github.com/OllieBerzs/draw-it/tree/develop/examples).

## Credits

This library uses these open source projects:

- [Ash](https://github.com/MaikKlein/ash) - Vulkan API support
- [Kenney](https://www.kenney.nl/assets) - Assets for examples and fonts
- [Serde](https://github.com/serde-rs/serde) - Deserialize assets
- [Tar](https://github.com/alexcrichton/tar-rs) - Unarchive assets
- [Notify](https://github.com/notify-rs/notify) - Hot-reload
- [Crossbeam](https://github.com/crossbeam-rs/crossbeam) - Hot-reload

Optional projects for extra functionality:

- [Image](https://github.com/image-rs/image) - Image loading
- [Winit](https://github.com/rust-windowing/winit) - OS Windowing
- [Imgui](https://github.com/Gekkio/imgui-rs) - Easy UI
- [Lazy Static](https://github.com/rust-lang-nursery/lazy-static.rs) - Profiling
