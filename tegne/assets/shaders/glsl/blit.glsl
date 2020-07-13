#define VERTEX_POSITION_MODELSPACE

void fragment() {
    out_color = texture(albedo, in_uv);
}
