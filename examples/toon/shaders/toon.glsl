// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// Toon shading shader

#define SHADOW

void fragment() {
    vec3 mesh_color = vec3(0.1, 0.3, 0.7);
    vec3 ambient_color = vec3(0.4, 0.4, 0.4);
    vec3 diffuse_color = vec3(1.0, 1.0, 1.0);
    vec3 specular_color = vec3(0.9, 0.9, 0.9);
    vec3 rim_color = vec3(1.0, 1.0, 1.0);
    float glossiness = 32.0;
    float rim_amount = 0.716;
    float rim_threshold = 0.1;

    Light light = world.lights[0];
    vec3 light_dir = normalize(-light.coords.xyz);
    vec3 view_dir = normalize(world.camera_position - in_worldspace_position.xyz);
    vec3 normal = normalize(in_normal);

    // diffuse
    float NdotL = dot(normal, light_dir);
    float diffuse_intensity = smoothstep(0.0, 0.01, NdotL * shadow(light));
    vec3 diffuse = diffuse_color * diffuse_intensity;

    // specular
    vec3 half_vector = normalize(light_dir + view_dir);
    float NdotH = dot(normal, half_vector);
    float specular_intensity = clamp(pow(NdotH * diffuse_intensity, glossiness * glossiness), 0.0, 1.0);
    specular_intensity = smoothstep(0.005, 0.01, specular_intensity);
    vec3 specular = specular_color * specular_intensity;

    // rim
    float rim_dot = 1.0 - dot(view_dir, normal);
    float rim_intensity = rim_dot * pow(NdotL, rim_threshold);
    rim_intensity = smoothstep(rim_amount - 0.01, rim_amount + 0.01, rim_intensity);
    vec3 rim = rim_color * rim_intensity;

    out_color = vec4(mesh_color * (ambient_color + diffuse + specular + rim), 1.0);
}
