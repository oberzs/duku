layout(location = 0) out vec4 out_color;

void fragment() {
    float value = sin(world.time * 2.0) * 0.5 + 0.5;
    out_color = vec4(1.0, value, 1.0, 1.0);
}
