// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// default vertex shader

void vertex() {
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
