#define SRGB
#define VERTEX_COLOR_SRGB

layout(location = 0) out vec4 out_color;

void fragment() {
    vec4 tex_color = texture(sampler2D(textures[object.albedo_index], sampler_m), in_uv);
    out_color = tex_color * in_color;
}
