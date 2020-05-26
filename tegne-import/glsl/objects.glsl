// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// glsl shader objects

struct Light {
    vec4 coords;
    vec4 color;
};

layout(set = 0, binding = 0) uniform WorldObject {
    mat4 cam_mat;
    mat4 light_mat;
    Light lights[4];
    vec3 cam_pos;
    float time;
    int shadow_index;
} world;

layout(set = 1, binding = 0) uniform MaterialObject {
    vec3 albedo_tint;
    float font_width;
    vec3 font_border_tint;
    float font_edge;
    vec2 font_border_offset;
    float font_border_width;
    float font_border_edge;
    vec4 arg_1;
    vec4 arg_2;
    vec4 arg_3;
    vec4 arg_4;
} material;

layout(push_constant) uniform Constants {
    mat4 model_mat;
    int albedo_index;
} object;