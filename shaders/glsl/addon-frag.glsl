// Oliver Berzs
// https://github.com/oberzs/duku

// fragment shader variables

layout(location = 0) in vec2 in_uv;
layout(location = 1) in vec4 in_color;
layout(location = 2) flat in uint in_texture;
layout(location = 3) in vec3 in_local_position;
layout(location = 4) in vec3 in_world_position;
layout(location = 5) in vec3 in_view_position;
layout(location = 6) in vec4 in_clip_position;
layout(location = 7) in vec4 in_shadow_position[4];
layout(location = 11) in mat3 in_tbn;

layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler samplers[6];
layout(set = 2, binding = 2) uniform textureCube cubemaps[100];
layout(set = 3, binding = 0) uniform texture2D shadow_maps[4];

// sampler combinations
#define sampler_lr samplers[0] // linear, repeat
#define sampler_lb samplers[1] // linear, border
#define sampler_le samplers[2] // linear, edge
#define sampler_nr samplers[3] // nearest, repeat
#define sampler_nb samplers[4] // nearest, border
#define sampler_ne samplers[5] // nearest, edge

// texture lookup functions
vec4 tex(uint index, vec2 uv) {
    return texture(sampler2D(textures[index], samplers[object.sampler_index]), uv);
}

vec4 cub(uint index, vec3 dir) {
    return texture(samplerCube(cubemaps[index], sampler_le), dir);
}

vec2 tex_size(uint index) {
    return textureSize(sampler2D(textures[index], samplers[object.sampler_index]), 0);
}
