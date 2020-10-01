// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// shape rendering shader

#define DEPTH test_and_write
#define CULL disabled
#define SHAPE filled_triangles

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = in_color;
}
