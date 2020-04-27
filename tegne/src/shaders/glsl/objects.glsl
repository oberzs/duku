struct Light {
    vec4 position;
    vec4 color;
};

layout(push_constant) uniform Constants {
    mat4 model;
    int albedo_index;
} CONSTANTS;

layout(set = 0, binding = 0) uniform WorldObject {
    mat4 view;
    mat4 proj;
    mat4 light_matrix;
    Light lights[4];
    vec4 view_pos;
    float time;
} W_UNIFORMS;

layout(set = 1, binding = 0) uniform MaterialObject {
    vec4 albedo_tint;
} M_UNIFORMS;