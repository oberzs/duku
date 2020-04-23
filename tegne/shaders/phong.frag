#version 450
#include "frag.glsl"

vec3 calc_dir_light(Light light, vec3 normal, vec3 view_dir);
vec3 calc_point_light(Light light, vec3 normal, vec3 view_dir, vec3 pos);

void main() {
    vec3 norm = normalize(NORMAL);
    vec3 view_dir = normalize(VIEW_POS - POSITION);

    vec3 lighting = vec3(0.0, 0.0, 0.0);
    for (int i = 0; i < 4; i++) {
        if (LIGHTS[i].position.w == 0.0) {
            lighting += calc_dir_light(LIGHTS[i], norm, view_dir);
        } else if (LIGHTS[i].position.w == 1.0) {
            lighting += calc_point_light(LIGHTS[i], norm, view_dir, POSITION);
        }
    }
    

    COLOR = texture(ALBEDO, UV) * ALBEDO_TINT * vec4(lighting, 1.0);
}

vec3 calc_dir_light(Light light, vec3 normal, vec3 view_dir) {
    vec3 light_dir = normalize(-light.position.xyz);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    // combine results
    vec3 ambient = 0.1 * light.color.xyz;
    vec3 diffuse = diff * light.color.xyz;
    vec3 specular = 0.5 * spec * light.color.xyz;
    return (ambient + diffuse + specular);
}

vec3 calc_point_light(Light light, vec3 normal, vec3 view_dir, vec3 pos) {
    vec3 light_dir = normalize(light.position.xyz - pos);
    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
    // attenuation
    float distance = length(light.position.xyz - pos);
    float attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * (distance * distance));
    // combine results
    vec3 ambient = 0.1 * light.color.xyz;
    vec3 diffuse = diff * light.color.xyz;
    vec3 specular = 0.5 * spec * light.color.xyz;
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}