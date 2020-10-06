// Oliver Berzs
// https://github.com/oberzs/draw-it

// shadow receiving calculation

float tex_sm(int index, vec3 uvc) {
    return texture(sampler2DShadow(shadow_maps[index], sampler_cm), uvc);
}

float tex_pcfsm(int index, vec3 uvc) {
    float depth = 0.0;
    vec2 texel = 1.0 / textureSize(sampler2DShadow(shadow_maps[index], sampler_cm), 0);
    float softness = 0.5 + world.shadow_pcf;
    for (float x = -softness; x <= softness; x += 1.0) {
        for (float y = -softness; y <= softness; y += 1.0) {
            vec2 offset = vec2(x, y) * texel;
            depth += texture(sampler2DShadow(shadow_maps[index], sampler_cm), vec3(uvc.xy + offset, uvc.z));
        }
    }
    depth /= 16.0;
    return pow(depth, 2.2);
}

float tex_shadow(int index, vec3 uvc) {
    if (world.shadow_pcf == 2.0) {
        return tex_sm(index, uvc);
    } else {
        return tex_pcfsm(index, uvc);
    }
}

vec3 tex_coord(int index, float bias) {
    vec4 coord = in_shadow_position[index];
    coord.y = -coord.y;
    coord.z -= bias;
    coord.xyz / coord.w;

    vec2 uv = coord.xy * 0.5 + 0.5;
    return vec3(uv.x, uv.y, coord.z);
}

float shadow(Light light) {
    float depth = in_clip_position.z;
    float blend_margin = world.shadow_cascades[3] * 0.05;

    // choose shadow map
    int cascade;
    if (depth < world.shadow_cascades[0]) {
        cascade = 0;
    } else if (depth < world.shadow_cascades[1]) {
        cascade = 1;
    } else if (depth < world.shadow_cascades[2]) {
        cascade = 2;
    } else {
        cascade = 3;
    }

    vec3 light_dir = normalize(-light.coords);
    vec3 normal = normalize(in_normal);
    float bias = world.shadow_bias * tan(acos(dot(normal, light_dir)));

    vec3 coord = tex_coord(cascade, bias);

    if (coord.z > 1.0) {
        return 0.0;
    } else {
        // blend between side-by-side cascades
        float blend = smoothstep(-blend_margin, 0.0, depth - world.shadow_cascades[cascade]);
        if (blend == 0.0) {
            return tex_shadow(cascade, coord);
        } else {
            int next_cascade = min(3, cascade + 1);
            vec3 next_coord = tex_coord(next_cascade, bias);

            float shadow = tex_shadow(cascade, coord);
            float next_shadow = tex_shadow(next_cascade, next_coord);

            return mix(shadow, next_shadow, blend);
        }

    }
}
