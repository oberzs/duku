// Oliver Berzs
// https://github.com/oberzs/duku

// bitmap font shader

#define DEPTH write
#define CULL disabled
#define SHAPE filled_triangles 

layout(location = 0) out vec4 frag_color;

void fragment() {
    float value = tex(in_texture, in_uv).r;
    frag_color = in_color * vec4(1.0, 1.0, 1.0, value);
}
