#define VERTEX_POSITION_MODELSPACE

void fragment() {
    out_color = texture(sampler2D(framebuffer, samplers[object.sampler_index]), in_uv) * vec4(material.albedo_tint, 1.0);
}
