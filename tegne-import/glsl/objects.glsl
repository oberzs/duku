struct Light {
    vec4 coords;
    vec4 color;
};

layout(set = 0, binding = 0) uniform WorldObject {
    mat4 cam_mat;
    vec3 cam_pos;
    mat4 light_mat;
    Light lights[4];
    int shadow_index;
    float time;
} world;

layout(set = 1, binding = 0) uniform MaterialObject {
    vec4 albedo_tint;
} material;

layout(push_constant) uniform Constants {
    mat4 model_mat;
    int albedo_index;
} object;