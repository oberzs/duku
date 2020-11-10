// Oliver Berzs
// https://github.com/oberzs/duku

// fullscreen texture blitting

#define DEPTH disabled
#define CULL disabled
#define SHAPE filled_triangles

#define VERTEX_LOCAL_POSITION

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = tex(int(material.arg_1.a), in_uv);
}
