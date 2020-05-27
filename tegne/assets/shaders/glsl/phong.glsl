#define PHONG

#ifdef VERTEX
void vertex() {
    gl_Position = world.cam_mat * object.model_mat * vec4(in_position, 1.0);
    out_position = vec3(object.model_mat * vec4(in_position, 1.0));
    out_ls_position = world.light_mat * vec4(out_position, 1.0);
    out_normal = mat3(transpose(inverse(object.model_mat))) * in_normal;
    out_color = in_color;
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
void fragment() {
    out_color = texture(sharp_albedo, in_uv) * in_color;
}
#endif
