// Oliver Berzs
// https://github.com/oberzs/draw-it

// skybox sampling shader

#define DEPTH test
#define CULL disabled
#define SHAPE filled_triangles

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = texture(samplerCube(skybox, sampler_em), in_local_position);
}

void vertex() {
    vec4 local_position = vec4(in_local_position, 1.0);
    vec4 world_position = object.local_to_world * local_position;
    vec4 view_position = world.world_to_view * world_position;
    vec4 clip_position = world.view_to_clip * view_position;
    out_local_position = local_position.xyz;
    gl_Position = clip_position.xyww;
}
