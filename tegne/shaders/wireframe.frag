#version 450
#include "frag.glsl"

void main() {
    float c = sin(TIME * 2.0) * 0.5 + 0.5;
    COLOR = vec4(1.0, c, 1.0, 1.0);
}