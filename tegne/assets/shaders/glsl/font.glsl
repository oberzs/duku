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
    float width = 0.5;
    float edge = 0.1;

    float dist = 1.0 - texture(albedo, in_uv).a;
    float alpha = 1.0 - smoothstep(width, width + edge, dist);

    if (alpha < 0.01) {
        discard;
    }

    out_color = vec4(0.0, 0.0, 0.0, alpha);
}
#endif