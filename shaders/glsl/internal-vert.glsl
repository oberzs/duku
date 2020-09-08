// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// vertex shader variables

layout(location = 0) in vec3 in_modelspace_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_color;
layout(location = 4) in uint in_texture;

layout(location = 0) out vec3 out_normal;
layout(location = 1) out vec2 out_uv;
layout(location = 2) out vec4 out_color;
layout(location = 3) flat out uint out_texture;
layout(location = 4) out vec3 out_modelspace_position;
layout(location = 5) out vec3 out_worldspace_position;
layout(location = 6) out vec4 out_screenspace_position;
layout(location = 7) out vec4 out_lightspace_position[4];
