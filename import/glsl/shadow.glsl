// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// shadow receiving calculation

#if defined(SHADOW)

float tex_sm(int index, vec3 uvc) {
    return texture(sampler2DShadow(shadow_maps[index], sampler_cm), uvc);
}

float tex_pcfsm(int index, vec3 uvc, float softer) {
    float depth = 0.0;
    vec2 texel = 1.0 / textureSize(sampler2DShadow(shadow_maps[index], sampler_cm), 0);
    float softness = 0.5 + softer;
    for (float x = -softness; x <= softness; x += 1.0) {
        for (float y = -softness; y <= softness; y += 1.0) {
            vec2 offset = vec2(x, y) * texel;
            depth += texture(sampler2DShadow(shadow_maps[index], sampler_cm), vec3(uvc.xy + offset, uvc.z));
        }
    }
    depth /= 16.0;
    return pow(depth, 2.2);
}

vec3 tex_coord(int index) {
    vec4 coord = in_lightspace_position[index];
    coord.y = -coord.y;

    vec2 uv = (coord.xy / coord.w) * 0.5 + 0.5;
    float depth = (coord.z + 0.0003) / coord.w;

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
    } else if (depth < world.cascade_splits[2]) {
        cascade = 2;
    } else {
        cascade = 3;
    }

    vec3 coord = tex_coord(cascade);

    if (coord.z > 1.0) {
        return 0.0;
    } else {
        // blend between side-by-side cascades
        float blend = smoothstep(-blend_margin, 0.0, depth - world.cascade_splits[cascade]);
        if (blend == 0.0) {
            return tex_pcfsm(cascade, coord);
        } else {
            int next_cascade = min(3, cascade + 1);
            vec3 next_coord = tex_coord(next_cascade);

            float shadow = tex_pcfsm(cascade, coord);
            float next_shadow = tex_pcfsm(next_cascade, next_coord);

            return mix(shadow, next_shadow, blend);
        }

    }
}

#endif
