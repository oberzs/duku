layout(set = 2, binding = 0) uniform texture2D textures[100];
layout(set = 2, binding = 1) uniform sampler samplers[2];

#define linear_repeat_sampler samplers[0]
#define linear_clamp_sampler samplers[1]
#define albedo sampler2D(textures[object.albedo_index], linear_repeat_sampler)
#define shadow_map sampler2D(textures[4], linear_clamp_sampler)

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;

layout(location = 0) out vec3 out_position;
layout(location = 1) out vec3 out_normal;
layout(location = 2) out vec2 out_uv;
layout(location = 3) out vec4 out_ls_position;