#define VERTEX_POSITION_WORLDSPACE

void fragment() {
    out_color = texture(albedo, in_uv) * vec4(material.albedo_tint, 1.0);
}
