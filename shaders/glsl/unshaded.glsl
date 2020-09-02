// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// unshaded shader

#define DEPTH test_and_write
#define CULL back
#define SHAPE filled_triangles

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = tex(0, in_uv) * in_color;
}
