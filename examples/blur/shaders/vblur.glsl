#define VERTEX_POSITION_MODELSPACE

const float KERNEL[7] = {0.00598, 0.060626, 0.241843, 0.383103, 0.241843, 0.060626, 0.00598};

void fragment() {
    float ts = 1.0 / tex_size(object.albedo_index).y;
    vec4 color = vec4(0.0);
    for (int i = -3; i < 3; ++i) {
        color += tex(object.albedo_index, in_uv + vec2(0.0, ts * i)) * KERNEL[i + 3];
    }
    out_color = color;
}
