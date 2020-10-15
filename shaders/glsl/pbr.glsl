// Oliver Berzs
// https://github.com/oberzs/draw-it

// PBR shader for 4 light sources

#define DEPTH test_and_write
#define SHAPE filled_triangles
#define CULL back

#define SHADOW

layout(location = 0) out vec4 out_color;

const float PI = 3.14159265359;

// calculates reflected vs refracted light amount
// uses Fresnel-Schlick approximation
vec3 fresnel_schlick(float cos_theta, vec3 F0) {
    return F0 + (1.0 - F0) * pow(1.0 - cos_theta, 5.0);
}

float distribution_ggx(vec3 N, vec3 H, float roughness) {
    float a = roughness * roughness;
    float a2 = a * a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH * NdotH;

    float num = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

float geometry_schlick_ggx(float NdotV, float roughness) {
    float r = roughness + 1.0;
    float k = (r * r) / 8.0;

    float num = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

float geometry_smith(vec3 N, vec3 V, vec3 L, float roughness) {
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = geometry_schlick_ggx(NdotV, roughness);
    float ggx1 = geometry_schlick_ggx(NdotL, roughness);

    return ggx1 * ggx2;
}

void fragment() {
    vec3 albedo = material.arg_1.rgb;
    float metallic = material.arg_2.r;
    float roughness = material.arg_2.g;
    float ao = 1.0;

    // calculate normal and view direction
    vec3 N = normalize(in_normal);
    vec3 V = normalize(world.camera_position - in_world_position);

    vec3 Lo = vec3(0.0);

    for (int i = 0; i < 4; ++i) {
        Light light = world.lights[i];
    
        float shdw = 1.0;
        if (light.type == LIGHT_TYPE_MAIN) {
            shdw = shadow(light);
        }

        vec3 L = vec3(0.0);
        vec3 radiance = vec3(0.0);

        if (light.type == LIGHT_TYPE_DIRECTIONAL || light.type == LIGHT_TYPE_MAIN) {
            L = normalize(-light.coords.xyz);
            radiance = light.color.xyz;
        } else if (light.type == LIGHT_TYPE_POINT) {
            L = normalize(light.coords.xyz - in_world_position);
            float distance = length(light.coords.xyz - in_world_position);
            float attenuation = 1.0 / (distance * distance);
            radiance = light.color.xyz * attenuation;
        }

        vec3 H = normalize(V + L);

        // calculate how much surface reflects
        // when looking directly at it
        // non-metallics get constant 0.04, metalics get their albedo
        vec3 F0 = vec3(0.04);
        F0 = mix(F0, albedo, metallic);
        vec3 F = fresnel_schlick(max(dot(H, V), 0.0), F0);

        float NDF = distribution_ggx(N, H, roughness);
        float G = geometry_smith(N, V, L, roughness);

        // calculate Cook-Torrance BRDF
        vec3 numerator = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
        vec3 specular = numerator / max(denominator, 0.001);

        // calculate light's contribution
        vec3 kS = F; // reflected part (specular)
        vec3 kD = vec3(1.0) - kS; // refracted part (diffuse)
        kD *= 1.0 - metallic;

        float NdotL = max(dot(N, L), 0.0);
        Lo += (kD * albedo / PI + specular) * radiance * NdotL * shdw;
    }

    vec3 ambient = vec3(0.03) * albedo * ao;
    vec3 color = ambient + Lo;

    out_color = vec4(color, 1.0);
}
