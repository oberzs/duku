// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// phong lighting calculations

#if defined(PHONG)
vec3 calc_dir_light(Light light, vec3 cam_dir, float shadow) {
    vec3 normal = normalize(in_normal);
    vec3 light_dir = normalize(-light.coords.xyz);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(cam_dir, reflect_dir), 0.0), 32);
    // combine results
    vec3 ambient = 0.1 * light.color.xyz;
    vec3 diffuse = diff * light.color.xyz;
    vec3 specular = 0.5 * spec * light.color.xyz;
    return (ambient + (diffuse + specular) * (1.0 - shadow));
}

vec3 calc_point_light(Light light, vec3 cam_dir, vec3 pos, float shadow) {
    vec3 normal = normalize(in_normal);
    vec3 light_dir = normalize(light.coords.xyz - pos);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(cam_dir, reflect_dir), 0.0), 32);
    // attenuation
    float distance = length(light.coords.xyz - pos);
    float attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * (distance * distance));
    // combine results
    vec3 ambient = 0.1 * light.color.xyz;
    vec3 diffuse = diff * light.color.xyz;
    vec3 specular = 0.5 * spec * light.color.xyz;
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (ambient + (diffuse + specular) * (1.0 - shadow));
}

float calc_shadow(Light light) {
    float shadow = 0.0;

    // choose shadow map
    int shadow_index;
    if (in_screenspace_position.z < world.cascade_splits.x) {
        shadow_index = 0;
    } else if (in_screenspace_position.z < world.cascade_splits.y) {
        shadow_index = 1;
    } else {
        shadow_index = 2;
    }

    vec3 projected = in_lightspace_position[shadow_index].xyz / in_lightspace_position[shadow_index].w;

    vec3 normal = normalize(in_normal);
    vec2 uv = projected.xy * 0.5 + 0.5;
    float current_depth = projected.z;
    vec3 light_dir = normalize(-light.coords.xyz);

    // depth bias
    float bias = max(0.001 * (1.0 - dot(normal, light_dir)), 0.0001);

    // PCF
    int strength = 3;
    vec2 texel_size = 1.0 / textureSize(sampler2D(shadow_maps[shadow_index], sampler_c), 0);
    for (int x = -strength; x <= strength; ++x) {
        for (int y = -strength; y <= strength; ++y) {
            vec2 pcf_uv = uv + vec2(x, y) * texel_size;
            float pcf_depth = texture(sampler2D(shadow_maps[shadow_index], sampler_c), pcf_uv).r;
            shadow += current_depth - bias > pcf_depth ? 1.0 : 0.0;
        }
    }
    shadow /= pow(strength * 2.0 + 1.0, 2.0);

    if (current_depth > 1.0) {
        shadow = 0.0;
    }

    return shadow;
}

vec4 phong() {
    vec3 cam_dir = normalize(world.camera_position - in_modelspace_position.xyz);

    // shadows
    float shadow = calc_shadow(world.lights[0]);

    // ligthing
    vec3 lighting = vec3(0.0, 0.0, 0.0);
    for (int i = 0; i < 4; i++) {
        Light light = world.lights[i];
        if (light.coords.w == 0.0) {
            lighting += calc_dir_light(light, cam_dir, shadow);
        } else if (light.coords.w == 1.0) {
            lighting += calc_point_light(light, cam_dir, in_modelspace_position.xyz, shadow);
        }
    }

    return vec4(lighting, 1.0);
}
#endif
