#include "../objects.glsl"

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_ls_position;

layout(location = 0) out vec4 out_color;

void main() {
    float value = sin(world.time * 2.0) * 0.5 + 0.5;
    out_color = vec4(1.0, value, 1.0, 1.0);
}