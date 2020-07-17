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
    float variance = max(moments.y - moments.x * moments.x, world.variance_min);

    float d = compare - moments.x;
    float low = world.shadow_low + world.shadow_low * index;
    float p_max = linstep(low, 1.0, variance / (variance + d * d));

    return min(max(p, p_max), 1.0);
}

vec3 tex_coord(int index) {
    vec4 coord = in_lightspace_position[index];
    coord.y = -coord.y;

    vec2 uv = (coord.xy / coord.w) * 0.5 + 0.5;
    float depth = coord.z / coord.w;

    return vec3(uv.x, uv.y, depth);
}

float shadow(Light light) {
    float depth = in_screenspace_position.z;
    float blend_margin = world.cascade_splits[2] * 0.05;

    // choose shadow map
    int cascade;
    if (depth < world.cascade_splits[0]) {
        cascade = 0;
    } else if (depth < world.cascade_splits[1]) {
        cascade = 1;
    } else {
        cascade = 2;
    }

    vec3 coord = tex_coord(cascade);

    if (coord.z > 1.0) {
        return 0.0;
    } else {
        // blend between side-by-side cascades
        float blend = smoothstep(-blend_margin, 0.0, depth - world.cascade_splits[cascade]);
        if (blend == 0.0) {
            return tex_vsm(cascade, coord.xy, coord.z);
        } else {
            float shadow = tex_vsm(cascade, coord.xy, coord.z);

            int next_cascade = min(2, cascade + 1);
            vec3 next_coord = tex_coord(next_cascade);
            float next_shadow = tex_vsm(next_cascade, next_coord.xy, next_coord.z);

            return mix(shadow, next_shadow, blend);
        }

    }
}

#endif
