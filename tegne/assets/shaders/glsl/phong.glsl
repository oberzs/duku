// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// basic phong-blinn shader for 1 light source

#define PHONG
#define SHADOW

void fragment() {
    Light light = world.lights[0];
    float shadow = shadow(light);
    vec3 phong_light = phong(light);
    vec3 ambient = 0.1 * light.color.rgb;
    vec4 lighting = vec4(ambient + phong_light * shadow, 1.0);

    vec4 color_mix = texture(albedo, in_uv) * vec4(material.albedo_tint, 1.0) * in_color;

    out_color = color_mix * lighting;
}
