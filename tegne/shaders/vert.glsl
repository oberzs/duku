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

layout(location = 0) in vec3 IN_POSITION;
layout(location = 1) in vec3 IN_NORMAL;
layout(location = 2) in vec2 IN_UV;

layout(location = 0) out vec3 OUT_POSITION;
layout(location = 1) out vec3 OUT_NORMAL;
layout(location = 2) out vec2 OUT_UV;
layout(location = 3) out vec3 OUT_LIGHT_SPACE_POS;

#define POSITION gl_Position
#define MODEL CONSTANTS.model
#define ALBEDO_INDEX CONSTANTS.albedo_index
#define VIEW W_UNIFORMS.view
#define PROJ W_UNIFORMS.proj
#define LIGHT_MATRIX W_UNIFORMS.light_matrix
#define TIME W_UNIFORMS.time