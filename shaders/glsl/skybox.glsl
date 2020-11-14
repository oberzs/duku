// Oliver Berzs
// https://github.com/oberzs/duku

// skybox sampling shader

#define DEPTH test
#define CULL disabled
#define SHAPE filled_triangles

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = cub(world.skybox_index, in_local_position);
}

void vertex() {
    vec4 local_position = vec4(in_local_position, 1.0);
    vec4 clip_position = world.view_to_clip * world.world_to_view * object.local_to_world * local_position;
    out_local_position = local_position.xyz;
    gl_Position = clip_position.xyww;
}
