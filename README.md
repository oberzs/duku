<h1 align="center">Duku 🎨</h1>

<div align="center">
  <!-- Version -->
  <span>
    <img src="https://img.shields.io/badge/version-0.1.0-green?style=flat-square" alt="Version" />
  </span>
  <!-- Docs -->
  <a href="https://docs.rs/duku">
    <img src="https://img.shields.io/badge/docs-0.1.0-blue?style=flat-square" alt="Docs" />
  </a>
  <!-- Build status -->
  <span>
    <img src="https://img.shields.io/github/workflow/status/oberzs/duku/Full%20Build?style=flat-square" alt="Build Status" />
  </span>
  <!-- Rust Version -->
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/rust-1.48.0-orange?style=flat-square" alt="Rust Version" />
  </a>
  <!-- License -->
  <a href="https://github.com/oberzs/duku/blob/release/LICENSE">
    <img src="https://img.shields.io/github/license/oberzs/duku?style=flat-square" alt="License" />
  </a>
</div>

<div align="center">
  <a href="#simple-example">Simple Example</a> •
  <a href="#usage">Usage</a> •
  <a href="#features">Features</a>
</div>

<br>

A Rust crate for creating graphic experiences, with a focus on ease of use and beginner friendliness.
Also helpful for visualizing algorithms when learning Rust.

This creative coding library draws a lot of inspiration from [p5.js].

## Simple Example

This example draws a cube in the center of the window, rotating and coloring it based on the time that has passed.

```rust
use duku::Camera;
use duku::Duku;
use duku::Hsb;
use duku::Light;
use duku::Result;
use std::time::Instant;

fn main() -> Result<()> {
  // create duku context and window
  let (mut duku, window) = Duku::windowed(500, 500)?;

  // create 3D camera with 90 fov
  let camera = Camera::perspective(90);

  // create directional light
  let light = Light::directional("#ffffff", [-1.0, -1.0, 1.0]);

  // start timer for rotation and color
  let timer = Instant::now();

  // start window loop
  window.while_open(move |_| {
      // start drawing on window
      duku.draw(Some(&camera), |t| {
          // setup scene
          t.background("#ababab");
          t.light(light);

          // get elapsed time since start
          let elapsed = timer.elapsed().as_secs_f32();

          // transform scene
          let angle = elapsed * 45.0;
          t.rotate_x(angle);
          t.rotate_y(angle);
          t.translate_z(2.0);

          // draw cube
          let hue = (elapsed * 60.0) as u16;
          t.tint(Hsb::new(hue, 70, 80));
          t.cube([1.0, 1.0, 1.0]);
      });
  });

  Ok(())
}
```

Want more? Check out these other [examples].

---

## Usage

To use this crate, add this dependency to your `Cargo.toml` file.

```toml
[dependencies]
duku = "0.1.0"
```

---

## Features

- **Supports** - Windows and Linux X11
- **Vulkan** - uses the [Vulkan SDK]
- **3D** - mesh rendering with materials and shaders
- **2D** - shape, texture and text rendering with batching
- **PBR** - uses a PBR material/shader system
- **Shadows** - uses PCF shadow maps
- **Text** - uses [Fira Mono] font
- **Shaders** - adding custom shaders is documented [here](https://github.com/oberzs/duku/tree/release/DC_DOCS.md)

#### Additional Features

This crate supports additional features that you can add
to your dependency in your `Cargo.toml` file.

```toml
[dependencies]
duku = { ... , features = ["feature-name"] }
```

The features include:

| Name     | Default | Uses           | Description                           |
| -------- | ------- | -------------- | ------------------------------------- |
| `window` | yes     | [winit]        | adds OS window creation support       |
| `png`    | no      | [png]          | adds png file loading support         |
| `jpeg`   | no      | [jpeg-decoder] | adds jpeg file loading support        |
| `gltf`   | no      | [gltf]         | adds gltf file loading support        |
| `glsl`   | no      | [shaderc]      | adds custom glsl file loading support |
| `log`    | no      | n/a            | adds informational logs               |

[p5.js]: https://p5js.org/
[examples]: https://github.com/oberzs/duku/tree/release/examples
[crates.io]: https://crates.io
[vulkan sdk]: https://vulkan.lunarg.com/
[fira mono]: https://fonts.google.com/specimen/Fira+Mono?query=fira
[png]: https://github.com/image-rs/image-png
[jpeg-decoder]: https://github.com/image-rs/jpeg-decoder
[gltf]: https://github.com/gltf-rs/gltf
[shaderc]: https://github.com/google/shaderc-rs
[winit]: https://github.com/rust-windowing/winit
