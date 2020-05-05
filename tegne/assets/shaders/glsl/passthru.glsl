#ifdef VERTEX
void vertex() {
    gl_Position = vec4(in_position, 1.0);
    out_position = in_position;
    out_normal = in_normal;
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
void fragment() {
    out_color = texture(albedo, in_uv) * material.albedo_tint;
}
#endif