<h1 align="center">Dc Docs</h1>

<div align="center">
  <strong>Shader compilation for Duku</strong>
</div>

<br />

<div align="center">
  <a href="#simple-example">Simple Example</a> •
  <a href="#using-compiler">Using Compiler</a> •
  <a href="#writing-shaders">Writing Shaders</a>
</div>

## Simple Example

```glsl
// file: shader.glsl

// configure shader
#define DEPTH test_and_write
#define SHAPE filled_triangles
#define CULL back

// specify shader output
layout(location = 0) out vec4 out_color;

// main fragment shader function
void fragment() {
  // get color from bound material
  vec3 color = material.arg_1.rgb;

  // output color to framebuffer
  out_color = vec4(color, 1.0);
}
```

Can be compiled to `.spirv` or imported directly as a `.glsl`

---

## Using Compiler

To compile a duku `.spirv` file, you can use the CLI utilicy
`dc` downloaded from the release page or compiling the `dc` binary target from source with cargo.

```bash
$ dc test.glsl
```

Running this command will compile the `test.glsl` file into
`test.spirv` and put it relative to the input file.

#### Options

There are some additional options available to customize the compilation process.

| Argument          | Short | Effect                                                     |
| ----------------- | :---: | ---------------------------------------------------------- |
| `--out some/path` | `-o`  | will put the output in the specified directory `some/path` |
| `--help`          | `-h`  | will print out the help information                        |
| `--no-color`      |  n/a  | will disable the colors in the output                      |
| `--version`       |  n/a  | will print out the compiler version                        |

---

## Writing Shaders

The language used for writing shaders is [glsl]
with some custom additions, which will be specified
below.

#### Configuration

The first thing you need to define in each shader are these
usage options.

| Name    | Options                                        | Effect                                            |
| ------- | ---------------------------------------------- | ------------------------------------------------- |
| `DEPTH` | `test`, `write`, `test_and_write`              | defines how depth tests are gonna be applied      |
| `SHAPE` | `filled_triangles`, `lined_triangles`, `lines` | defines how the vertices are gonna be interpreted |
| `CULL`  | `back`, `front`, `disabled`                    | defines what culling method is applied            |

Example:

```glsl
#define DEPTH test_and_write
#define SHAPE filled_triangles
#define CULL back
```

#### Entry Functions

In [glsl] the entry functions for both vertex and fragment
shaders are usually defined as `main`, but because for duku
both vertex and fragment shaders are written in the same file
they are defined as `vertex` and `fragment` like so:

```glsl
void fragment() {
  // output fragments
}

void vertex() {
  // output vertices
}
```

As the vertex shader is optional (and is usually ommited),
global constants outside these functions will
be available only in the fragment shader.

[glsl]: https://www.khronos.org/opengl/wiki/Core_Language_(GLSL)

#### Global Objects

Global objects provide data that has
been passed from the Rust code.

`world` defines values that are set for each framebuffer target.

| Name               | Type       | Description                                                   |
| ------------------ | ---------- | ------------------------------------------------------------- |
| `world_to_view`    | `mat4`     | matrix that transforms coordinate from world to view space    |
| `view_to_clip`     | `mat4`     | matrix that transforms coordinate from view to clip space     |
| `world_to_shadow`  | `mat4[4]`  | matrices that transform coordinate from world to shadow space |
| `camera_position`  | `vec3`     | camera position in world space                                |
| `skybox_index`     | `uint`     | index for current skybox                                      |
| `time`             | `float`    | time since the start of context creation                      |
| `lights`           | `Light[4]` | lights that are in the scene                                  |
| `ambient_color`    | `vec3`     | ambient light's color                                         |
| `shadow_splits`    | `vec4`     | shadow map split values                                       |
| `shadow_texels`    | `vec4`     | shadow map texel sizes                                        |
| `shadow_diameters` | `vec4`     | shadow map area diameters                                     |
| `shadow_pcf`       | `float`    | shadow map softness value                                     |
| `max_white_point`  | `float`    | value used to do tone mapping                                 |

`material` defines values that are set for each material.

| Name    | Type   | Description        |
| ------- | ------ | ------------------ |
| `arg_1` | `vec4` | user defined value |
| `arg_2` | `vec4` | user defined value |
| `arg_3` | `vec4` | user defined value |
| `arg_4` | `vec4` | user defined value |
| `arg_5` | `vec4` | user defined value |
| `arg_6` | `vec4` | user defined value |
| `arg_7` | `vec4` | user defined value |
| `arg_8` | `vec4` | user defined value |

`object` defines values that are set for each draw call.

| Name             | Type   | Description                                                 |
| ---------------- | ------ | ----------------------------------------------------------- |
| `local_to_world` | `mat4` | matrix that transforms coordinate from local to world space |
| `sampler_index`  | `uint` | sampler index                                               |

The `Light` type is defines like this:

```glsl
struct Light {
  vec3 coords;
  int type;
  vec4 color;
};
```

#### Fragment Shader Definitions

These global values and functions
are defined for each fragment shader.

| Name                 | Type                       | Description                                |
| -------------------- | -------------------------- | ------------------------------------------ |
| `in_uv`              | `in vec2`                  | Vertex UV coordinate                       |
| `in_color`           | `in vec4`                  | Vertex color                               |
| `in_texture`         | `flat in uint`             | Vertex texture                             |
| `in_local_position`  | `in vec3`                  | Vertex local space coordinate              |
| `in_world_position`  | `in vec3`                  | Vertex world space coordinate              |
| `in_view_position`   | `in vec3`                  | Vertex view space coordinate               |
| `in_clip_position`   | `in vec4`                  | Vertex clip space coordinate               |
| `in_shadow_position` | `in vec4[4]`               | Vertex shadow space coordinates            |
| `in_tbn`             | `in mat3`                  | Vertex tangent-bitangent-normal matrix     |
| `textures`           | `uniform texture2D[100]`   | All loaded textures                        |
| `samplers`           | `uniform sampler[12]`      | All loaded samplers                        |
| `cubemaps`           | `uniform textureCube[100]` | All loaded cubemaps                        |
| `shadow_maps`        | `uniform texture2D[4]`     | Currently bound shadow maps                |
| `tex`                | `(uint, vec2) -> vec4`     | Samples a texture with the current sampler |
| `cub`                | `(uint, vec3 -> vec4`      | Samples a cubemap                          |
| `tex_size`           | `(uint) -> vec2`           | Gets the texture's size                    |

#### Vertex Shader Definitions

These global values and functions
are defined for each vertex shader.

| Name                  | Type            | Description                            |
| --------------------- | --------------- | -------------------------------------- |
| `in_local_position`   | `in vec3`       | Vertex local space coordinate          |
| `in_normal`           | `in vec3`       | Vertex normal direction                |
| `in_tangent`          | `in vec3`       | Vertex tangent direction               |
| `in_uv`               | `in vec2`       | Vertex UV coordinate                   |
| `in_color`            | `in vec4`       | Vertex color                           |
| `in_texture`          | `in uint`       | Vertex texture                         |
| `out_uv`              | `out vec2`      | Vertex UV coordinate                   |
| `out_color`           | `out vec4`      | Vertex color                           |
| `out_texture`         | `flat out uint` | Vertex texture                         |
| `out_local_position`  | `out vec3`      | Vertex local space coordinate          |
| `out_world_position`  | `out vec3`      | Vertex world space coordinate          |
| `out_view_position`   | `out vec3`      | Vertex view space coordinate           |
| `out_clip_position`   | `out vec4`      | Vertex clip space coordinate           |
| `out_shadow_position` | `out vec4[4]`   | Vertex shadow space coordinates        |
| `out_tbn`             | `out mat3`      | Vertex tangent-bitangent-normal matrix |

#### Additional Definitions

These global and functions can be imported
by defining a module like so:

```glsl
#define SRGB
```

| Name        | Module   | Type                                                   | Description                                                          |
| ----------- | -------- | ------------------------------------------------------ | -------------------------------------------------------------------- |
| `to_linear` | `SRGB`   | `(float) -> float`, `(vec3) -> vec3`, `(vec4) -> vec4` | converts value to linear color space                                 |
| `to_srgb`   | `SRGB`   | `(float) -> float`, `(vec3) -> vec3`, `(vec4) -> vec4` | converts value to sRGB color space                                   |
| `shadow`    | `SHADOW` | `(Light, vec3) -> float`                               | calculates the received shadow using the light and the normal vector |
