// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// variance shadow mapping write shader

void fragment() {
    float depth = gl_FragCoord.z;
    float dx = dFdx(depth);
    float dy = dFdy(depth);
    float moment_2 = depth * depth + 0.25 * (dx * dx + dy * dy);

    out_color = vec4(depth, moment_2, 0.0, 1.0);
}
