#include "vert.glsl"

void main() {
    POSITION = vec4(IN_POSITION, 1.0);
    OUT_POSITION = IN_POSITION;
    OUT_NORMAL = IN_NORMAL;
    OUT_UV = IN_UV;
}