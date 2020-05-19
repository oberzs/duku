#ifdef VERTEX
void vertex() {
    gl_Position = vec4(in_position, 1.0);
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
void fragment() {
    float percent = in_uv.x;
    vec3 color = mix(material.arg_1.rgb, material.arg_2.rgb, percent);
    out_color = vec4(color, 1.0);
}
#endif