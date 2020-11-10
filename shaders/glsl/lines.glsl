// Oliver Berzs
// https://github.com/oberzs/duku

// line rendering shader

#define DEPTH test_and_write
#define CULL disabled
#define SHAPE lines

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = in_color;
}
