#ifdef VERTEX
void vertex() {
    gl_Position = world.cam_mat * object.model_mat * vec4(in_position, 1.0);
    out_position = vec3(object.model_mat * vec4(in_position, 1.0));
    out_ls_position = world.light_mat * vec4(out_position, 1.0);
    out_normal = mat3(transpose(inverse(object.model_mat))) * in_normal;
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
void fragment() {
    float value = sin(world.time * 2.0) * 0.5 + 0.5;
    out_color = vec4(1.0, value, 1.0, 1.0);
}
#endif