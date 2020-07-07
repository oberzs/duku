#define PHONG

void fragment() {
    vec4 tint;
    if (in_screenspace_position.z < world.cascade_splits.x) {
        tint = vec4(0.5, 0.0, 0.0, 1.0);
    } else if (in_screenspace_position.z < world.cascade_splits.y) {
        tint = vec4(0.0, 0.5, 0.0, 1.0);
    } else {
        tint = vec4(0.0, 0.0, 0.5, 1.0);
    }

    out_color = texture(albedo, in_uv) * vec4(material.albedo_tint, 1.0) * in_color * phong() * tint;
}
