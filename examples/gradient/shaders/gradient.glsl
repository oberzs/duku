#define VERTEX_POSITION_MODELSPACE

void fragment() {
    vec3 color_1 = material.arg_1.rgb;
    vec3 color_2 = material.arg_2.rgb;
    float percent = in_uv.x;
    vec3 color = mix(color_1, color_2, percent);
    out_color = vec4(color, 1.0);
}
