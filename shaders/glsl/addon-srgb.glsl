// Oliver Berzs
// https://github.com/oberzs/duku

// sRGB and linear conversions

float to_linear(float value) {
    float s = clamp(value, 0.0, 1.0);
    float cutoff = 0.04045;
    float gamma = 2.2;

    if (s <= cutoff) {
        return s / 12.92;
    } else {
        return pow((s + 0.055) / 1.055, gamma);
    }
}

vec3 to_linear(vec3 color) {
    float r = to_linear(color.r);
    float g = to_linear(color.g);
    float b = to_linear(color.b);

    return vec3(r, g, b);
}

vec4 to_linear(vec4 color) {
    return vec4(to_linear(color.rgb), color.a);
}

float to_srgb(float value) {
    float l = clamp(value, 0.0, 1.0);
    float cutoff = 0.0031308;
    float gamma = 2.2;

    if (l <= cutoff) {
        return l * 12.92;
    } else {
        return 1.055 * pow(l, 1.0 / gamma) - 0.055;
    }
}

vec3 to_srgb(vec3 color) {
    float r = to_srgb(color.r);
    float g = to_srgb(color.g);
    float b = to_srgb(color.b);

    return vec3(r, g, b);
}

vec4 to_srgb(vec4 color) {
    return vec4(to_srgb(color.rgb), color.a);
}
