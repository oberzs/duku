// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// base vertex shader

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


void main() {
    vec4 modelspace_position = vec4(in_modelspace_position, 1.0);
    vec4 worldspace_position = object.model_matrix * modelspace_position;
    vec4 screenspace_position = world.world_matrix * worldspace_position;

    out_modelspace_position = modelspace_position.xyz;
    out_worldspace_position = worldspace_position.xyz;
    out_screenspace_position = screenspace_position;
    out_lightspace_position[0] = world.light_matrices[0] * worldspace_position;
    out_lightspace_position[1] = world.light_matrices[1] * worldspace_position;
    out_lightspace_position[2] = world.light_matrices[2] * worldspace_position;
    out_lightspace_position[3] = world.light_matrices[3] * worldspace_position;

    out_normal = mat3(transpose(inverse(object.model_matrix))) * in_normal;
    out_color = {{out_color}};
    out_uv = in_uv;
    out_texture = in_texture;

    gl_Position = {{out_position}};
}
