// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// fragment shader variables

layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler samplers[3];

#define linear_repeat_sampler samplers[0]
#define linear_clamp_sampler samplers[1]
#define nearest_repeat_sampler samplers[2]
#define albedo sampler2D(textures[object.albedo_index], linear_repeat_sampler)
#define sharp_albedo sampler2D(textures[object.albedo_index], nearest_repeat_sampler)
#define shadow_map sampler2D(textures[world.shadow_index], linear_clamp_sampler)

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_ls_position;
