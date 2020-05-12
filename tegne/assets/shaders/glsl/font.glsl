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
    vec3 tint = material.albedo_tint;
    float width = material.font_width;
    float edge = material.font_edge;
    float border_width = material.font_border_width;
    float border_edge = material.font_border_edge;
    vec3 border_tint = material.font_border_tint;
    vec2 border_offset = material.font_border_offset;

    float dist = 1.0 - texture(albedo, in_uv).a;
    float alpha = 1.0 - smoothstep(width, width + edge, dist);

    float border_dist = 1.0 - texture(albedo, in_uv + border_offset).a;
    float border_alpha = 1.0 - smoothstep(border_width, border_width + border_edge, border_dist);

    float overall_alpha = alpha + (1.0 - alpha) * border_alpha;
    vec3 overall_tint = mix(border_tint, tint, alpha / overall_alpha);

    if (overall_alpha < 0.01) {
        discard;
    }

    out_color = vec4(overall_tint, overall_alpha);
}
#endif