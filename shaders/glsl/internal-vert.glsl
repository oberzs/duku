// Oliver Berzs
// https://github.com/oberzs/draw-it

// vertex shader variables

layout(location = 0) in vec3 in_local_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_color;
layout(location = 4) in uint in_texture;

layout(location = 0) out vec3 out_normal;
layout(location = 1) out vec2 out_uv;
layout(location = 2) out vec4 out_color;
layout(location = 3) flat out uint out_texture;
layout(location = 4) out vec3 out_local_position;
layout(location = 5) out vec3 out_world_position;
layout(location = 6) out vec3 out_view_position;
layout(location = 7) out vec4 out_clip_position;
layout(location = 8) out vec4 out_shadow_position[4];
