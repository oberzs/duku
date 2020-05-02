#include "../objects.glsl"

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_ls_position;

layout(location = 0) out vec4 out_color;

void main() {
    out_color = texture(albedo, in_uv) * material.albedo_tint;
}