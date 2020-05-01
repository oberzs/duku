#include "../objects.glsl"

layout(location = 0) in vec3 in_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;

layout(location = 0) out vec3 out_position;
layout(location = 1) out vec3 out_normal;
layout(location = 2) out vec2 out_uv;
layout(location = 3) out vec4 out_ls_position;

void main() {
    gl_Position = world.light_mat * object.model_mat * vec4(0.5 * in_position, 1.0);
}