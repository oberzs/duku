struct Light {
    vec4 coords;
    vec4 color;
};

layout(set = 0, binding = 0) uniform WorldObject {
    mat4 cam_mat;
    vec3 cam_pos;
    mat4 light_mat;
    Light lights[4];
    float time;
} world;

layout(set = 1, binding = 0) uniform MaterialObject {
    vec4 albedo_tint;
} material;

layout(push_constant) uniform Constants {
    mat4 model_mat;
    int albedo_index;
} object;

layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler linear_sampler;

#define albedo sampler2D(textures[object.albedo_index], linear_sampler)
#define shadow_map sampler2D(textures[4], linear_sampler)