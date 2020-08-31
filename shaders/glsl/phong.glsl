// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// basic phong-blinn shader for 1 light source

#define DEPTH test_and_write
#define SHAPE filled_triangles
#define CULL back

#define SHADOW

layout(location = 0) out vec4 out_color;

void fragment() {
    vec4 albedo_color = vec4(material.arg_1.rgb, 1.0);

    Light light = world.lights[0];
    vec3 normal = normalize(in_normal);
    vec3 light_dir = normalize(-light.coords);
    vec3 cam_dir = normalize(world.camera_position - in_modelspace_position.xyz);

    // received shadows
    float shadow = shadow(light);

    // diffuse shading
    float diff = max(dot(normal, light_dir), 0.0);
    vec3 diffuse = diff * light.color.rgb;

    // specular shading
    vec3 reflect_dir = reflect(-light_dir, normal);
    float spec = pow(max(dot(cam_dir, reflect_dir), 0.0), 32);
    vec3 specular = 0.5 * spec * light.color.rgb;

    // ambient light
    vec3 ambient = 0.1 * light.color.rgb;

    // combine results
    vec4 lighting = vec4(ambient + (diffuse) * shadow, 1.0);
    vec4 color_mix = tex(object.albedo_index, in_uv) * albedo_color * in_color;

    out_color = lighting * color_mix;
}
