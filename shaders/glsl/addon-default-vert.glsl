// Oliver Berzs
// https://github.com/oberzs/draw-it

// default vertex shader

void vertex() {
    vec4 local_position = vec4(in_local_position, 1.0);
    vec4 world_position = object.local_to_world * local_position;
    vec4 view_position = world.world_to_view * world_position;
    vec4 clip_position = world.view_to_clip * view_position;

    out_local_position = local_position.xyz;
    out_world_position = world_position.xyz;
    out_view_position = view_position.xyz;
    out_clip_position = clip_position;

    out_shadow_position[0] = world.world_to_shadow[0] * world_position;
    out_shadow_position[1] = world.world_to_shadow[1] * world_position;
    out_shadow_position[2] = world.world_to_shadow[2] * world_position;
    out_shadow_position[3] = world.world_to_shadow[3] * world_position;

    mat3 local_to_normal = mat3(transpose(inverse(object.local_to_world)));
    vec3 normal = local_to_normal * in_normal;
    vec3 tangent = local_to_normal * in_tangent;
    tangent = normalize(tangent - dot(tangent, normal) * normal);
    vec3 bitangent = cross(tangent, normal);

    out_tbn = mat3(tangent, bitangent, normal);
    out_color = {{out_color}};
    out_uv = in_uv;
    out_texture = in_texture;

    gl_Position = {{out_position}};
}
