#include "vert.glsl"

void main() {
    vec4 pos = LIGHT_MATRIX * MODEL * vec4(IN_POSITION, 1.0);
    POSITION = pos;
    OUT_POSITION = pos.xyz;
    OUT_NORMAL = IN_NORMAL;
    OUT_UV = IN_UV;
}