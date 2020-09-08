// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// fragment shader variables

layout(location = 0) in vec3 in_normal;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec4 in_color;
layout(location = 3) flat in uint in_texture;
layout(location = 4) in vec3 in_modelspace_position;
layout(location = 5) in vec3 in_worldspace_position;
layout(location = 6) in vec4 in_screenspace_position;
layout(location = 7) in vec4 in_lightspace_position[4];

layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler samplers[12];
layout(set = 2, binding = 2) uniform textureCube skybox;
layout(set = 3, binding = 0) uniform texture2D shadow_maps[4];

// sampler combinations
#define sampler samplers[0] // linear, repeat, mipmaps
#define sampler_m samplers[1] // linear, repeat, no-mipmaps
#define sampler_c samplers[2] // linear, clamp, mipmaps
#define sampler_cm samplers[3] // linear, clamp, no-mipmaps
#define sampler_e samplers[4] // linear, clamp-edge, mipmaps
#define sampler_em samplers[5] // linear, clamp-edge, no-mipmaps
#define sampler_n samplers[6] // nearest, repeat, mipmaps
#define sampler_nm samplers[7] // nearest, repeat, no-mipmaps
#define sampler_nc samplers[8] // nearest, clamp, mipmaps
#define sampler_ncm samplers[9] // nearest, clamp, no-mipmaps
#define sampler_ne samplers[10] // nearest, clamp-edge, mipmaps
#define sampler_nem samplers[11] // nearest, clamp-edge, no-mipmaps

// texture lookup functions
vec4 tex(uint index, vec2 uv) {
    return texture(sampler2D(textures[index], samplers[object.sampler_index]), uv);
}

vec2 tex_size(uint index) {
    return textureSize(sampler2D(textures[index], samplers[object.sampler_index]), 0);
}
