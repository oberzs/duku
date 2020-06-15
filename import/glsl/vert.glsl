// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// base vertex shader

layout(location = 0) in vec3 in_worldspace_position;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;
layout(location = 3) in vec4 in_color;

layout(location = 0) out vec3 out_normal;
layout(location = 1) out vec2 out_uv;
layout(location = 2) out vec4 out_color;
layout(location = 3) out vec4 out_screenspace_position;
layout(location = 4) out vec4 out_modelspace_position;
layout(location = 5) out vec4 out_worldspace_position;
layout(location = 6) out vec4 out_lightspace_position;


void main() {
    out_normal = mat3(transpose(inverse(object.model_matrix))) * in_normal;
    out_uv = in_uv;
#if defined(VERTEX_COLOR_SRGB)
    out_color = srgb_to_linear_color(in_color);
#else
    out_color = in_color;
#endif
    out_worldspace_position = vec4(in_worldspace_position, 1.0);
    out_modelspace_position = object.model_matrix * out_worldspace_position;
    out_screenspace_position = world.world_matrix * out_modelspace_position;
    out_lightspace_position = world.light_matrix * out_modelspace_position;
#if defined(VERTEX_POSITION_LIGHTSPACE)
    gl_Position = out_lightspace_position;
#elif defined(VERTEX_POSITION_WORLDSPACE)
    gl_Position = out_worldspace_position;
#else
    gl_Position = out_screenspace_position;
#endif
}
