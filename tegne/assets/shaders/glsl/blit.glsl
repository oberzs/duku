#define VERTEX_POSITION_MODELSPACE

void fragment() {
    out_color = tex(object.albedo_index, in_uv);
}
