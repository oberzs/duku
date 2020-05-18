#ifdef VERTEX
void vertex() {
    gl_Position = vec4(in_position, 1.0);
    out_position = in_position;
    out_normal = in_normal;
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
float get_dist_to_scene(vec3 pos) {
    vec4 sphere = vec4(0, 1, 6, 1);

    float sphere_dist = length(pos - sphere.xyz) - sphere.w;
    float plane_dist = pos.y;

    float dist = min(sphere_dist, plane_dist);
    return dist;
}

float ray_march(vec3 ray_orig, vec3 ray_dir) {
    int MAX_STEPS = 100;
    float MAX_DIST = 100.0;
    float SURF_DIST = 0.01;

    float dist_orig = 0.0;

    for (int i = 0; i < MAX_STEPS; i++) {
        vec3 pos = ray_orig + ray_dir * dist_orig;
        float dist_scene = get_dist_to_scene(pos);
        dist_orig += dist_scene;

        if (dist_orig > MAX_DIST || dist_scene < SURF_DIST) break;
    }

    return dist_orig;
}

vec3 get_normal(vec3 pos) {
    float dist = get_dist_to_scene(pos);
    vec2 e = vec2(0.01, 0.0);

    vec3 normal = dist - vec3(
        get_dist_to_scene(pos - e.xyy),
        get_dist_to_scene(pos - e.yxy),
        get_dist_to_scene(pos - e.yyx));

    return normalize(normal);
}

float get_light(vec3 pos) {
    float SURF_DIST = 0.01;

    vec3 light_pos = vec3(0.0, 5.0, 6.0);
    light_pos.xz += vec2(sin(world.time), cos(world.time));
    vec3 light_dir = normalize(light_pos - pos);
    vec3 normal = get_normal(pos);

    float light = clamp(dot(normal, light_dir), 0.0, 1.0);
    float dist = ray_march(pos + normal * SURF_DIST, light_dir);
    if (dist < length(light_pos - pos)) light *= 0.1;
    return light;
}

void fragment() {
    vec2 uv = in_uv * 2.0 - 1.0;
    uv.y *= -1.0;

    vec3 ray_orig = vec3(0.0, 1.0, 0.0);
    vec3 ray_dir = normalize(vec3(uv.x, uv.y, 1.0));

    vec3 color = vec3(0.0);

    float dist = ray_march(ray_orig, ray_dir);
    vec3 pos = ray_orig + ray_dir * dist;
    float light = get_light(pos);

    color = vec3(light);

    out_color = vec4(color, 1.0);
}
#endif