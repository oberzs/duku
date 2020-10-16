// Oliver Berzs
// https://github.com/oberzs/draw-it

// PBR shader for 4 light sources

#define DEPTH test_and_write
#define SHAPE filled_triangles
#define CULL back

#define SHADOW

layout(location = 0) out vec4 out_color;

const float PI = 3.14159265359;

// calculates reflected light percentage
// uses Fresnel-Schlick approximation
vec3 fresnel_schlick(float cos_theta, vec3 direct_refl) {
    return direct_refl + (1.0 - direct_refl) * pow(1.0 - cos_theta, 5.0);
}

float distribution_ggx(float dot_nh, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float dot_nh2 = dot_nh * dot_nh;

    float num = a2;
    float denom = (dot_nh2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

float geometry_schlick_ggx(float dot_nv, float roughness) {
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;

    float num = dot_nv;
    float denom = dot_nv * (1.0 - k) + k;

    return num / denom;
}

float geometry_smith(float dot_nv, float dot_nl, float roughness) {
    float ggx2 = geometry_schlick_ggx(dot_nv, roughness);
    float ggx1 = geometry_schlick_ggx(dot_nl, roughness);

    return ggx1 * ggx2;
}

void fragment() {
    vec4 albedo_tex = tex(int(material.arg_1.a), in_uv);
    vec4 roughness_tex = tex(int(material.arg_2.b), in_uv);
    vec4 metalness_tex = tex(int(material.arg_2.a), in_uv);
    float ambient_occlusion = tex(int(material.arg_3.r), in_uv).r;
    vec3 albedo = material.arg_1.rgb * albedo_tex.rgb;
    float metalness = material.arg_2.r * metalness_tex.r;
    float roughness = material.arg_2.g * roughness_tex.r;

    // calculate normal and view direction
    vec3 normal = normalize(in_normal);
    vec3 view_dir = normalize(world.camera_position - in_world_position);

    vec3 light_amount = vec3(0.0);

    for (int i = 0; i < 4; ++i) {
        Light light = world.lights[i];
    
        float occlusion = 1.0;
        if (light.type == LIGHT_TYPE_MAIN) {
            occlusion = shadow(light);
        }

        vec3 light_dir = vec3(0.0);
        vec3 radiance = vec3(0.0);

        if (light.type == LIGHT_TYPE_DIRECTIONAL || light.type == LIGHT_TYPE_MAIN) {
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
        float dot_hv = max(dot(half_dir, view_dir), 0.0);
        float dot_nl = max(dot(normal, light_dir), 0.0);
        float dot_nv = max(dot(normal, view_dir), 0.0);
        float dot_nh = max(dot(normal, half_dir), 0.0);

        // calculate how much surface reflects
        // when looking directly at it
        // non-metallics get constant 0.04, metalics get their albedo
        vec3 direct_refl = vec3(0.04);
        direct_refl = mix(direct_refl, albedo, metalness);

        // calculate light's specular (reflected)
        // and diffuse (refracted) parts
        vec3 specular_part = fresnel_schlick(dot_hv, direct_refl);
        vec3 diffuse_part = vec3(1.0) - specular_part;
        diffuse_part *= 1.0 - metalness;

        float NDF = distribution_ggx(dot_nh, roughness);
        float G = geometry_smith(dot_nv, dot_nl, roughness);

        // calculate Cook-Torrance BRDF
        vec3 num = NDF * G * specular_part;
        float denom = 4.0 * dot_nv * dot_nl;
        vec3 specular = num / max(denom, 0.001);
        vec3 diffuse = diffuse_part * albedo / PI;

        light_amount += (diffuse + specular) * radiance * dot_nl * occlusion;
    }

    vec3 ambient = vec3(0.03) * albedo * ambient_occlusion;
    vec3 color = ambient + light_amount;

    // Reinhard tonemapping
    color = color / (color + vec3(1.0));

    out_color = vec4(color, 1.0);
}
