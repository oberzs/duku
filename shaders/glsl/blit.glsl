// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// framebuffer blitting shader

#define DEPTH disabled
#define CULL back
#define SHAPE filled_triangles

#define VERTEX_LOCAL_POSITION

layout(location = 0) out vec4 out_color;

void fragment() {
    // out_color = tex(object.albedo_index, in_uv);
    out_color = tex(0, in_uv);
}
