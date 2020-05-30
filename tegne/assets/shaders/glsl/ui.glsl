#define SRGB

#ifdef VERTEX
void vertex() {
    gl_Position = world.cam_mat * object.model_mat * vec4(in_position, 1.0);
    out_color = srgb_to_linear_color(in_color);
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
void fragment() {
    vec4 tex_color = texture(sampler2D(textures[object.albedo_index], sampler_m), in_uv);
    out_color = tex_color * in_color;
}
#endif
