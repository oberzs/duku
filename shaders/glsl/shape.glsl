// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// shape rendering shader

#define DEPTH test_and_write
#define CULL disabled
#define SHAPE filled_triangles

layout(location = 0) out vec4 out_color;

void fragment() {
    uint sindex = uint(in_normal.x);
    vec4 tex_color = texture(sampler2D(textures[in_texture], samplers[sindex]), in_uv);
    out_color = tex_color * in_color;
}

void vertex() {
    out_normal = in_normal;
    out_color = in_color;
    out_uv = in_uv;
    out_texture = in_texture;

    gl_Position = world.view_to_clip 
        * world.world_to_view 
        * object.local_to_world 
        * vec4(in_local_position, 1.0);
}
