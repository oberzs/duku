// Oliver Berzs
// https://github.com/oberzs/duku

// PBR shader for 4 light sources

#define DEPTH test_and_write
#define SHAPE filled_triangles
#define CULL back

#define SHADOW
#define SRGB

layout(location = 0) out vec4 out_color;

const float PI = 3.14159265359;

// calculates a color's luminance
float luminance(vec3 color) {
    return dot(color, vec3(0.2126f, 0.7152f, 0.0722f));
}

// changes the color's luminance to the specified one
vec3 change_luminance(vec3 color, float lum) {
    float lum_in = luminance(color);
    return color * (lum / lum_in);
}

// uses Extended Reinhard (Luminance Tone Map)
// tone maps the input color
vec3 tone_map(vec3 color) {
    float exposure = world.exposure;
    float lum = luminance(color);
    float num = lum * (1.0 + (lum / (exposure * exposure)));
    float new_lum = num / (1.0 + lum);
    return change_luminance(color, new_lum);
}

// uses Fresnel-Schlick approximation
// calculates the reflected lights contribution (also Fresnel effect)
vec3 specular_part(float h_dot_v, vec3 base_refl) {
    return base_refl + (1.0 - base_refl) * pow(1.0 - h_dot_v, 5.0);
}

// uses Trowbridge-Reitz GGX approximation
// calculates what proportion of microfacets
// align with bisecting vector
float normal_distribution(float n_dot_h, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float denom = n_dot_h * n_dot_h * (a2 - 1.0) + 1.0;
    denom = PI * denom * denom;
    return a2 / max(denom, 0.0000001);
}

// uses Smith's method and Schlick-GGX geometry approximation
// calculates the amount of microfacet self-shadowing
float self_shadowing(float n_dot_v, float n_dot_l, float roughness) {
    // Schlick-GGX
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;
    float ggx2 = n_dot_v / (n_dot_v * (1.0 - k) + k);
    float ggx1 = n_dot_l / (n_dot_l * (1.0 - k) + k);

    // Smith's method
    return ggx1 * ggx2;
}

void fragment() {
    vec4 albedo_tex = tex(int(material.a.a), in_uv);
    vec4 met_rough_tex = tex(int(material.b.b), in_uv);
    float ambient_occlusion = tex(int(material.b.a), in_uv).r;
    vec3 emissive = tex(int(material.c.g), in_uv).rgb * material.d.rgb;
    vec3 albedo = material.a.rgb * albedo_tex.rgb * object.tint_color;
    float metalness = material.b.r * met_rough_tex.b;
    float roughness = material.b.g * met_rough_tex.g;

    // calculate normal and view direction
    vec3 normal = tex(int(material.c.r), in_uv).xyz * (255.0 / 128.0) - 1.0;
    normal = normalize(in_tbn * normal);
    vec3 view_dir = normalize(world.camera_position - in_world_position);

    // calculate how much surface reflects
    // when looking directly at it
    // non-metallics get constant 0.04, metalics get their albedo
    vec3 base_refl = mix(vec3(0.04), albedo, metalness);

    vec3 light_amount = vec3(0.0);

    // calculate shadow casting contribution
    float shadow_occlusion = 1.0;
    if (world.shadow_light_index < 4) {
        uint i = world.shadow_light_index;
        shadow_occlusion *= shadow(world.lights[i], normal);
    }

    for (int i = 0; i < 4; ++i) {
        Light light = world.lights[i];

        vec3 light_dir = vec3(0.0);
        vec3 radiance = vec3(0.0);

        if (light.type == LIGHT_TYPE_DIRECTIONAL) {
            light_dir = normalize(-light.coords.xyz);
            radiance = light.color.xyz;
        } else if (light.type == LIGHT_TYPE_POINT) {
            float distance = length(light.coords.xyz - in_world_position);
            float attenuation = 1.0 / (distance * distance);

            radiance = light.color.xyz * attenuation;
            light_dir = normalize(light.coords.xyz - in_world_position);
        }

        vec3 half_dir = normalize(view_dir + light_dir);

        // calculate angles between vectors
        float h_dot_v = max(dot(half_dir, view_dir), 0.0);
        float n_dot_l = max(dot(normal, light_dir), 0.0);
        float n_dot_v = max(dot(normal, view_dir), 0.0);
        float n_dot_h = max(dot(normal, half_dir), 0.0);

        // calculate light's specular (reflected)
        // and diffuse (refracted) parts
        vec3 s_part = specular_part(h_dot_v, base_refl);
        vec3 d_part = (vec3(1.0) - s_part) * (1.0 - metalness);

        float nd = normal_distribution(n_dot_h, roughness);
        float ss = self_shadowing(n_dot_v, n_dot_l, roughness);

        // calculate Cook-Torrance BRDF
        // only diffuse light gets the color from the material
        vec3 specular = (s_part * nd * ss) / max(0.001, 4.0 * n_dot_v * n_dot_l);
        vec3 diffuse = d_part * albedo / PI;

        light_amount += (diffuse + specular) * radiance * n_dot_l;
    }

    vec3 ambient = world.ambient_color * albedo;
    vec3 color = ambient + emissive + light_amount * ambient_occlusion * shadow_occlusion;

    // tone mapping
    color = tone_map(color);

    // convert to sRGB
    color = to_srgb(color);

    out_color = vec4(color, 1.0);
}
