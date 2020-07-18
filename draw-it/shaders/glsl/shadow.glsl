// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// variance shadow mapping write shader

layout(location = 0) out vec4 out_color;

void fragment() {
    float depth = gl_FragCoord.z;
    float dx = dFdx(depth);
    float dy = dFdy(depth);
    float moment_2 = depth * depth + 0.25 * (dx * dx + dy * dy);

    out_color = vec4(depth, moment_2, 0.0, 1.0);
}
