#define VERTEX_POSITION_MODELSPACE

layout(location = 0) out vec4 out_color;

void fragment() {
    out_color = tex(object.albedo_index, in_uv);
}
