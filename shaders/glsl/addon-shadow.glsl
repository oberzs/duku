// Oliver Berzs
// https://github.com/oberzs/duku

// shadow receiving calculation

vec3 calc_coord(int index) {
    vec4 coord = in_shadow_position[index];
    coord.y = -coord.y;

    vec2 uv = coord.xy * 0.5 + 0.5;
    return vec3(uv.x, uv.y, coord.z);
}

float calc_bias(int index, float dot_nl) {
    return (0.01 + world.shadow_texels[index] / dot_nl) * (1.0 / world.shadow_diameters[index]);
}

float tex_sm(int index, vec3 uvc, float dot_nl) {
    float bias = calc_bias(index, dot_nl);
    return texture(sampler2DShadow(shadow_maps[index], sampler_lb), vec3(uvc.xy, uvc.z - bias));
}

float tex_pcfsm(int index, vec3 uvc, float dot_nl) {
    float depth = 0.0;
    float bias = calc_bias(index, dot_nl);
    vec2 texel = 1.0 / textureSize(sampler2DShadow(shadow_maps[index], sampler_lb), 0);
    float softness = 0.5 + world.shadow_pcf;
    for (float x = -softness; x <= softness; x += 1.0) {
        for (float y = -softness; y <= softness; y += 1.0) {
            vec2 offset = vec2(x, y) * texel;
            float bias_mult = (1.0 + 0.5 * length(vec2(x, y)));
            depth += texture(sampler2DShadow(shadow_maps[index], sampler_lb), vec3(uvc.xy + offset, uvc.z - bias * bias_mult));
        }
    }
    depth /= 16.0;
    return pow(depth, 2.2);
}

float tex_shadow(int index, vec3 uvc, float dot_nl) {
    if (world.shadow_pcf == 2.0) {
        return tex_sm(index, uvc, dot_nl);
    } else {
        return tex_pcfsm(index, uvc, dot_nl);
    }
}

float shadow(Light light, vec3 normal) {
    float depth = in_view_position.z;
    float blend_margin = world.shadow_splits[3] * 0.01;

    // choose shadow map
    int curr_split;
    if (depth < world.shadow_splits[0]) {
        curr_split = 0;
    } else if (depth < world.shadow_splits[1]) {
        curr_split = 1;
    } else if (depth < world.shadow_splits[2]) {
        curr_split = 2;
    } else if (depth < world.shadow_splits[3]) {
        curr_split = 3;
    } else {
        return 1.0;
    }
    vec3 curr_coord = calc_coord(curr_split);

    vec3 light_dir = normalize(-light.coords);
    float dot_nl = max(0.1, dot(normal, light_dir));

    if (curr_coord.z > 1.0) {
        return 0.0;
    } else {
        // blend between side-by-side splits
        float blend = smoothstep(-blend_margin, 0.0, depth - world.shadow_splits[curr_split]);
        if (blend == 0.0) {
            return tex_shadow(curr_split, curr_coord, dot_nl);
        } else {
            int next_split = min(3, curr_split + 1);
            vec3 next_coord = calc_coord(next_split);

            float curr_shadow = tex_shadow(curr_split, curr_coord, dot_nl);
            float next_shadow = tex_shadow(next_split, next_coord, dot_nl);

            return mix(curr_shadow, next_shadow, blend);
        }

    }
}
