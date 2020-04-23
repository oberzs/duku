#version 450
#include "frag.glsl"

void main() {
    COLOR = texture(ALBEDO, UV) * ALBEDO_TINT;
}