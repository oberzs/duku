void fragment() {
    out_color = tex(object.albedo_index, in_uv) * in_color;
}
