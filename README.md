<h1 align="center">Duku Renderer 🎨</h1>

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

This Rust crate makes it easy to render 2D and 3D graphics.

## Simple Example

A simple example that initiates duku, sets up a 3D camera
and draws a cube on the screen.

```rust
use duku::Color;
use duku::Camera;
use duku::Duku;
use duku::Result;

fn main() -> Result<()> {
    // initialize duku and OS window with a size of 500x500
    let (mut duku, window) = Duku::builder().build_window(500, 500).build()?;

    // create a 3D perspective camera with an FOV of 90
    let mut camera = Camera::perspective_autosized(90);

    // move the camera to some location
    // and make it look at the center of the world
    camera.transform.move_by([2.0, 1.5, -2.0]);
    camera.transform.look_at([0.0, 0.0, 0.0]);

    // start up the main event loop
    window.main_loop(move |_| {
      // start drawing on the window using our camera
      duku.draw_on_window(Some(&camera), |target| {
            // set the background color to sky blue
            target.clear = Color::SKY_BLUE;

            // draw a cube at the center of the world
            target.draw_cube();
        });
    });

    Ok(())
}
```

This example uses the optional feature `window` for OS window creation.

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
- **No dependencies** - all Rust dependencies are optional

#### Optional Features

This crate supports additional optional features that you can add
to your dependency in your `Cargo.toml` file.

```toml
[dependencies]
duku = { ... , features = ["feature-name"] }
```

The optional features include:

| Name     | Uses           | Description                           |
| -------- | -------------- | ------------------------------------- |
| `window` | [winit]        | adds OS window creation support       |
| `png`    | [png]          | adds png file loading support         |
| `jpeg`   | [jpeg-decoder] | adds jpeg file loading support        |
| `gltf`   | [gltf]         | adds gltf file loading support        |
| `glsl`   | [shaderc]      | adds custom glsl file loading support |
| `log`    | n/a            | adds informational logs               |

[examples]: https://github.com/oberzs/duku/tree/release/examples
[crates.io]: https://crates.io
[vulkan sdk]: https://vulkan.lunarg.com/
[fira mono]: https://fonts.google.com/specimen/Fira+Mono?query=fira
[png]: https://github.com/image-rs/image-png
[jpeg-decoder]: https://github.com/image-rs/jpeg-decoder
[gltf]: https://github.com/gltf-rs/gltf
[shaderc]: https://github.com/google/shaderc-rs
[winit]: https://github.com/rust-windowing/winit
