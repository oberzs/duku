void fragment() {
    float percent = in_uv.x;
    vec3 color = mix(material.arg_1.rgb, material.arg_2.rgb, percent);
    out_color = vec4(color, 1.0);
}
