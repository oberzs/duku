// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// glsl shader objects

struct Light {
    vec3 coords;
    int type;
    vec4 color;
};

layout(set = 0, binding = 0) uniform World {
    mat4 world_matrix;
    Light lights[4];
    vec3 camera_position;
    float time;
    mat4 light_matrices[4];
    vec4 cascade_splits;
    float bias;
    float pcf;
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
    mat4 model_matrix;
    uint sampler_index;
} object;
