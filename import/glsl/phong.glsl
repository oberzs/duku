// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// phong lighting calculations

#if defined(PHONG)

vec3 phong_dir_light(Light light) {
    vec3 normal = normalize(in_normal);
    vec3 light_dir = normalize(light.coords.xyz);
    vec3 cam_dir = normalize(world.camera_position - in_modelspace_position.xyz);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(cam_dir, reflect_dir), 0.0), 32);
    // combine results
    vec3 diffuse = diff * light.color.xyz;
    vec3 specular = 0.5 * spec * light.color.xyz;
    return diffuse + specular;
}

vec3 phong_point_light(Light light) {
    vec3 normal = normalize(in_normal);
    vec3 pos = in_modelspace_position.xyz;
    vec3 light_dir = normalize(pos - light.coords.xyz);
    vec3 cam_dir = normalize(world.camera_position - pos);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(cam_dir, reflect_dir), 0.0), 32);
    // attenuation
    float distance = length(light.coords.xyz - pos);
    float attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * (distance * distance));
    // combine results
    vec3 diffuse = diff * light.color.xyz;
    vec3 specular = 0.5 * spec * light.color.xyz;
    diffuse *= attenuation;
    specular *= attenuation;
    return diffuse + specular;
}

vec3 phong(Light light) {
    if (light.coords.w == 0.0) {
        return phong_dir_light(light);
    } else if (light.coords.w == 1.0) {
        return phong_point_light(light);
    } else {
        return vec3(0.0, 0.0, 0.0);
    }
}
#endif
