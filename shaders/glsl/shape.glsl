// Oliver Berzs
// https://github.com/oberzs/duku

// shape rendering shader

#define DEPTH test_and_write
#define CULL disabled
#define SHAPE filled_triangles

#define SRGB

layout(location = 0) out vec4 out_color;

void fragment() {
    uint sindex = uint(in_local_position.x);
    vec4 tex_color = texture(sampler2D(textures[in_texture], samplers[sindex]), in_uv);
    out_color = to_srgb(tex_color) * in_color;
}

void vertex() {
    out_color = in_color;
    out_uv = in_uv;
    out_texture = in_texture;
    out_local_position = in_normal;

    gl_Position = world.view_to_clip 
        * world.world_to_view 
        * object.local_to_world 
        * vec4(in_local_position, 1.0);
}
