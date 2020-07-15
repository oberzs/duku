// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// basic phong-blinn shader for 1 light source

layout(location = 0) out vec4 out_color;

#define PHONG
#define SHADOW

void fragment() {
    vec4 albedo_color = vec4(material.arg_1.rgb, 1.0);

    Light light = world.lights[0];
    float shadow = shadow(light);
    vec3 phong_light = phong(light);
    vec3 ambient = 0.1 * light.color.rgb;
    vec4 lighting = vec4(ambient + phong_light * shadow, 1.0);

    vec4 color_mix = tex(object.albedo_index, in_uv) * albedo_color * in_color;

    out_color = color_mix * lighting;
}
