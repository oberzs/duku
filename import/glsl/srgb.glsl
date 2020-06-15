// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// sRGB and linear conversions

#if defined(SRGB)
float srgb_to_linear(float value) {
    float s = clamp(value, 0.0, 1.0);
    float cutoff = 0.04045;
    float gamma = 2.2;

    if (s <= cutoff) {
        return s / 12.92;
    } else {
        return pow((s + 0.055) / 1.055, gamma);
    }
}

float linear_to_srgb(float value) {
    float l = clamp(value, 0.0, 1.0);
    float cutoff = 0.0031308;
    float gamma = 2.2;

    if (l <= cutoff) {
        return l * 12.92;
    } else {
        return 1.055 * pow(l, 1.0 / gamma) - 0.055;
    }
}

vec4 srgb_to_linear_color(vec4 color) {
    float r = srgb_to_linear(color.r);
    float g = srgb_to_linear(color.g);
    float b = srgb_to_linear(color.b);

    return vec4(r, g, b, color.a);
}

vec4 linear_to_srgb_color(vec4 color) {
    float r = linear_to_srgb(color.r);
    float g = linear_to_srgb(color.g);
    float b = linear_to_srgb(color.b);

    return vec4(r, g, b, color.a);
}
#endif
