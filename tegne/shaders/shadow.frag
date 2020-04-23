#version 450
#include "frag.glsl"

void main() {
    float z = POSITION.z;
    COLOR = vec4(z, z, z, 1.0);
}