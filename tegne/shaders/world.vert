#version 450
#include "vert.glsl"

void main() {
    POSITION = PROJ * VIEW * MODEL * vec4(IN_POSITION, 1.0);
    OUT_POSITION = vec3(MODEL * vec4(IN_POSITION, 1.0));
    OUT_NORMAL = mat3(transpose(inverse(MODEL))) * IN_NORMAL;
    OUT_UV = IN_UV;
}