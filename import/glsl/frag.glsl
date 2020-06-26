// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// fragment shader variables

layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler samplers[8];
layout(set = 3, binding = 0) uniform texture2D shadow_maps[3];
layout(set = 4, binding = 0) uniform texture2D framebuffer;

// sampler combinations
#define sampler samplers[0] // linear, repeat, mipmaps
#define sampler_m samplers[1] // linear, repeat, no-mipmaps
#define sampler_c samplers[2] // linear, clamp, mipmaps
#define sampler_cm samplers[3] // linear, clamp, no-mipmaps
#define sampler_n samplers[4] // nearest, repeat, mipmaps
#define sampler_nm samplers[5] // nearest, repeat, no-mipmaps
#define sampler_nc samplers[6] // nearest, clamp, mipmaps
#define sampler_ncm samplers[7] // nearest, clamp, no-mipmaps

// textures
#define albedo sampler2D(textures[object.albedo_index], samplers[object.sampler_index])

layout(location = 0) in vec3 in_normal;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec4 in_color;
layout(location = 3) in vec3 in_modelspace_position;
layout(location = 4) in vec3 in_worldspace_position;
layout(location = 5) in vec4 in_screenspace_position;
layout(location = 6) in vec4 in_lightspace_position[4];
