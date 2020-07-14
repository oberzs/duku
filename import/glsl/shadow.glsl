// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// shadow receiving calculation

#if defined(SHADOW)

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

    vec3 normal = normalize(in_normal);
    vec3 light_dir = normalize(-light.coords.xyz);

    // depth bias
    float angle = acos(clamp(dot(normal, light_dir), 0.0, 1.0));
    float bias = world.bias * tan(angle);
    if (angle == 0.0) {
        bias = 0.01;
    } else {
        bias = clamp(bias, 0.0, 0.01);
    }

    vec4 shadow_coord = in_lightspace_position[shadow_index];

    // have to flip Y for some reason
    shadow_coord.y = -shadow_coord.y;

    vec2 uv = (shadow_coord.xy / shadow_coord.w) * 0.5 + 0.5;
    float depth = (shadow_coord.z - bias) / shadow_coord.w;

    float shadow = 0.0;
    int strength = 1;
    vec2 texel_size = 1.0 / textureSize(sampler2DShadow(shadow_maps[shadow_index], sampler_cm), 0);
    for (int x = -strength; x <= strength; x++) {
        for (int y = -strength; y <= strength; y++) {
            vec2 off = vec2(x, y) * texel_size;
            shadow += texture(sampler2DShadow(shadow_maps[shadow_index], sampler_cm), vec3(uv + off, depth));
        }
    }
    if (strength > 0) {
        shadow /= pow(strength * 2 + 1, 2);
    }

    if (depth > 1.0) {
        shadow = 0.0;
    }

    return shadow;
}

#endif
