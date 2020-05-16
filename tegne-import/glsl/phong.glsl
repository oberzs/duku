#ifdef PHONG
vec3 calc_dir_light(Light light, vec3 normal, vec3 cam_dir, float shadow) {
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

vec3 calc_point_light(Light light, vec3 normal, vec3 cam_dir, vec3 pos, float shadow) {
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

float calc_shadow(vec4 ls_position, vec3 normal, Light light) {
    float shadow = 0.0;

    vec3 proj_coords = ls_position.xyz / ls_position.w;
    vec2 uv = proj_coords.xy * 0.5 + 0.5;
    float current_depth = proj_coords.z;
    vec3 light_dir = normalize(-light.coords.xyz);

    // depth bias
    float bias = max(0.001 * (1.0 - dot(normal, light_dir)), 0.0001);

    // PCF
    vec2 texel_size = 1.0 / textureSize(shadow_map, 0);
    for (int x = -1; x <= 1; ++x) {
        for (int y = -1; y <= 1; ++y) {
            float pcf_depth = texture(shadow_map, uv + vec2(x, y) * texel_size).r;
            shadow += current_depth - bias > pcf_depth ? 1.0 : 0.0;
        }
    }
    shadow /= 9;

    if (current_depth > 1.0) {
        shadow = 0.0;
    }

    return shadow;
}

vec4 phong() {
    vec3 normal = normalize(in_normal);
    vec3 cam_dir = normalize(world.cam_pos - in_position);

    // shadows
    float shadow = calc_shadow(in_ls_position, normal, world.lights[0]);

    // ligthing
    vec3 lighting = vec3(0.0, 0.0, 0.0);
    for (int i = 0; i < 4; i++) {
        Light light = world.lights[i];
        if (light.coords.w == 0.0) {
            lighting += calc_dir_light(light, normal, cam_dir, shadow);
        } else if (light.coords.w == 1.0) {
            lighting += calc_point_light(light, normal, cam_dir, in_position, shadow);
        }
    }

    return vec4(lighting, 1.0);
}
#endif