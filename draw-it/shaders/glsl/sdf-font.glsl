layout(location = 0) out vec4 out_color;

void fragment() {
    vec3 tint = material.arg_1.rgb;
    float width = material.arg_1.w;
    float edge = material.arg_2.w;
    float border_width = material.arg_3.z;
    float border_edge = material.arg_3.w;
    vec3 border_tint = material.arg_2.rgb;
    vec2 border_offset = material.arg_3.xy;

    float dist = 1.0 - tex(object.albedo_index, in_uv).r;
    float alpha = 1.0 - smoothstep(width, width + edge, dist);

    float border_dist = 1.0 - tex(object.albedo_index, in_uv + border_offset).r;
    float border_alpha = 1.0 - smoothstep(border_width, border_width + border_edge, border_dist);

    float overall_alpha = alpha + (1.0 - alpha) * border_alpha;
    vec3 overall_tint = mix(border_tint, tint, alpha / overall_alpha);

    if (overall_alpha < 0.01) {
        discard;
    }

    out_color = vec4(overall_tint, overall_alpha);
}
