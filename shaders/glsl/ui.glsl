// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// imgui rendering shader

#define DEPTH disabled
#define CULL disabled
#define SHAPE filled_triangles 

#define SRGB
#define VERTEX_SRGB_COLOR

layout(location = 0) out vec4 out_color;

void fragment() {
    float value = texture(sampler2D(textures[0], sampler_m), in_uv).r;
    out_color = vec4(1.0, 1.0, 1.0, value) * in_color;
}
