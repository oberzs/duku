// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// fragment shader variables

layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler samplers[8];

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
#define shadow_map sampler2D(textures[world.shadow_index], sampler_c)

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_color;
layout(location = 4) in vec4 in_ls_position;
