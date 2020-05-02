#include "../objects.glsl"

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_ls_position;

layout(location = 0) out vec4 out_color;

vec3 calc_dir_light(Light light, vec3 normal, vec3 cam_dir, float shadow);
vec3 calc_point_light(Light light, vec3 normal, vec3 cam_dir, vec3 pos, float shadow);
float calc_shadow(vec4 ls_position);

void main() {
    vec3 normal = normalize(in_normal);
    vec3 cam_dir = normalize(world.cam_pos - in_position);
    float shadow = calc_shadow(in_ls_position);

    vec3 lighting = vec3(0.0, 0.0, 0.0);
    for (int i = 0; i < 4; i++) {
        Light light = world.lights[i];
        if (light.coords.w == 0.0) {
            lighting += calc_dir_light(light, normal, cam_dir, shadow);
        } else if (light.coords.w == 1.0) {
            lighting += calc_point_light(light, normal, cam_dir, in_position, shadow);
        }
    }

    out_color = texture(albedo, in_uv) * material.albedo_tint * vec4(lighting, 1.0);
    // out_color = texture(albedo, in_uv) * shadow;
}

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

float calc_shadow(vec4 ls_position) {
    // perspective divide
    vec3 proj_coords = ls_position.xyz / ls_position.w;

    // to [0, 1]
    vec2 uv = proj_coords.xy * 0.5 + 0.5;

    float closest_depth = texture(shadow_map, uv).r;
    float current_depth = proj_coords.z;
    float shadow = current_depth > closest_depth ? 1.0 : 0.0;

    return shadow;
}