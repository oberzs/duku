// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// bitmap font shader

#define DEPTH write
#define CULL back
#define SHAPE filled_triangles 

layout(location = 0) out vec4 out_color;

void fragment() {
    float sampled = tex(in_texture, in_uv).r;
    out_color = in_color * sampled;
}
