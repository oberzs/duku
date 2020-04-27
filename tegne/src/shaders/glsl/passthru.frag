#include "frag.glsl"

void main() {
    COLOR = texture(ALBEDO, UV) * ALBEDO_TINT;
}