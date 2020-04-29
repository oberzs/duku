#include "vert.glsl"

void main() {
    POSITION = LIGHT_MATRIX * MODEL * vec4(IN_POSITION, 1.0);
}