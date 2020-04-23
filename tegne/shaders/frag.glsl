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

layout(set = 2, binding = 0) uniform texture2D TEXTURES[100];
layout(set = 2, binding = 1) uniform sampler SAMPLER;

layout(location = 0) in vec3 POSITION;
layout(location = 1) in vec3 NORMAL;
layout(location = 2) in vec2 UV;
layout(location = 3) in vec3 LIGHT_SPACE_POS;

layout(location = 0) out vec4 COLOR;

#define ALBEDO_TINT M_UNIFORMS.albedo_tint
#define ALBEDO_INDEX CONSTANTS.albedo_index
#define ALBEDO sampler2D(TEXTURES[ALBEDO_INDEX], SAMPLER)
#define LIGHTS W_UNIFORMS.lights
#define VIEW_POS W_UNIFORMS.view_pos.xyz
#define TIME W_UNIFORMS.time