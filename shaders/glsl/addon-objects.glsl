// Oliver Berzs
// https://github.com/oberzs/duku

// glsl shader objects

struct Light {
    vec3 coords;
    int type;
    vec4 color;
};

layout(set = 0, binding = 0) uniform World {
    mat4 world_to_view;
    mat4 view_to_clip;
    Light lights[4];
    vec3 camera_position;
    float time;
    mat4 world_to_shadow[4];
    vec4 shadow_splits;
    vec4 shadow_texels;
    vec4 shadow_diameters;
    vec3 ambient_color;
    float shadow_pcf;
    uint skybox_index;
    float max_white_point;
} world;

layout(set = 1, binding = 0) uniform Material {
    vec4 arg_1;
    vec4 arg_2;
    vec4 arg_3;
    vec4 arg_4;
    vec4 arg_5;
    vec4 arg_6;
    vec4 arg_7;
    vec4 arg_8;
} material;

layout(push_constant) uniform Constants {
    mat4 local_to_world;
    uint sampler_index;
} object;

#define LIGHT_TYPE_MAIN 0
#define LIGHT_TYPE_DIRECTIONAL 1
#define LIGHT_TYPE_POINT 2
