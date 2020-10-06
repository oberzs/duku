// Oliver Berzs
// https://github.com/oberzs/draw-it

// bitmap font shader

#define DEPTH write
#define CULL back
#define SHAPE filled_triangles 

layout(location = 0) out vec4 frag_color;

void fragment() {
    float sampled = tex(in_texture, in_uv).r;
    frag_color = in_color * sampled;
}
