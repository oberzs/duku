#define SRGB
#define VERTEX_COLOR_SRGB

void fragment() {
    vec4 tex_color = texture(sampler2D(textures[object.albedo_index], sampler_m), in_uv);
    out_color = tex_color * in_color;
}
