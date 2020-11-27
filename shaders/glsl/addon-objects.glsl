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
    vec4 a;
    vec4 b;
    vec4 c;
    vec4 d;
    vec4 e;
    vec4 f;
    vec4 g;
    vec4 h;
} material;

layout(push_constant) uniform Constants {
    mat4 local_to_world;
    vec3 tint_color;
    uint sampler_index;
} object;

#define LIGHT_TYPE_MAIN 0
#define LIGHT_TYPE_DIRECTIONAL 1
#define LIGHT_TYPE_POINT 2
