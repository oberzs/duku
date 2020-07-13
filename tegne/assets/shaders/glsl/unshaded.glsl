void fragment() {
    out_color = texture(albedo, in_uv) * in_color;
}
