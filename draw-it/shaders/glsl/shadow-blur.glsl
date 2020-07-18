// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// variance shadow mapping blur shader

#define VERTEX_POSITION_MODELSPACE

layout(location = 0) out vec4 out_color;

void fragment() {
    vec2 blur_scale = vec2(object.model_matrix[0][0], object.model_matrix[0][1]);
    vec4 color = vec4(0.0);    						

    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * -3.0)) * 0.00598;
    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * -2.0)) * 0.060626;
    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * -1.0)) * 0.241843;
    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * 0.0)) * 0.383103;
    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * 1.0)) * 0.241843;
    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * 2.0)) * 0.060626;
    color += texture(sampler2D(shadow_maps[object.albedo_index], sampler_cm), in_uv + (blur_scale * 3.0)) * 0.00598;

    out_color = color;
}
