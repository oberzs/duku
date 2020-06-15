#define PHONG

void fragment() {
    int index = 2;
    vec4 tint = vec4(0.0, 0.0, 1.0, 1.0);
    if (gl_FragCoord.z < world.cascade_splits.x) {
        index = 0;
        tint = vec4(1.0, 0.0, 0.0, 1.0);
    } else if (gl_FragCoord.z < world.cascade_splits.y) {
        index = 1;
        tint = vec4(0.0, 1.0, 0.0, 1.0);
    }

    out_color = texture(albedo, in_uv) * vec4(material.albedo_tint, 1.0) * in_color * phong();
    // out_color = vec4(vec3(in_position.z), 1.0 );
}
