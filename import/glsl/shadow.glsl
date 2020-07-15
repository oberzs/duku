// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// shadow receiving calculation

#if defined(SHADOW)

float linstep(float low, float high, float v) {
    return clamp((v - low) / (high - low), 0.0, 1.0);
}

float tex_vsm(int index, vec2 uv, float compare) {
    vec2 moments = texture(sampler2D(shadow_maps[index], sampler_cm), uv).xy;

    float p = step(compare, moments.x);
    float variance = max(moments.y - moments.x * moments.x, 0.00002);

    float d = compare - moments.x;
    // might have to make the 0.2 bigger depending on cascade
    float p_max = linstep(0.2, 1.0, variance / (variance + d * d));

    return min(max(p, p_max), 1.0);
}

float shadow(Light light) {
    // choose shadow map
    int shadow_index;
    if (in_screenspace_position.z < world.cascade_splits[0]) {
        shadow_index = 0;
    } else if (in_screenspace_position.z < world.cascade_splits[1]) {
        shadow_index = 1;
    } else {
        shadow_index = 2;
    }

    vec4 shadow_coord = in_lightspace_position[shadow_index];
    shadow_coord.y = -shadow_coord.y;

    vec2 uv = (shadow_coord.xy / shadow_coord.w) * 0.5 + 0.5;
    float depth = shadow_coord.z / shadow_coord.w;

    if (depth > 1.0) {
        return 0.0;
    } else {
        return tex_vsm(shadow_index, uv, depth);
    }
}

#endif
