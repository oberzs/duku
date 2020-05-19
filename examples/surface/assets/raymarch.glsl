#ifdef VERTEX
void vertex() {
    gl_Position = vec4(in_position, 1.0);
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
const int MAX_STEPS = 100;
const float MAX_DIST = 100.0;
const float SURF_DIST = 0.001;

mat2 rotate(float a) {
    float s = sin(a);
    float c = cos(a);
    return mat2(c, -s, s, c);
}

float sphere(vec3 pos, vec3 c, float r) {
    float dist = length(pos - c) - r;
    return dist;
}

float capsule(vec3 pos, vec3 a, vec3 b, float r) {
    vec3 ab = b - a;
    vec3 ap = pos - a;

    float t = dot(ab, ap) / dot(ab, ab);
    t = clamp(t, 0.0, 1.0);

    vec3 c = a + t * ab;
    float dist = length(pos - c) - r;
    return dist;
}

float cylinder(vec3 pos, vec3 a, vec3 b, float r) {
    vec3 ab = b - a;
    vec3 ap = pos - a;

    float t = dot(ab, ap) / dot(ab, ab);
    vec3 c = a + t * ab;
    float x = length(pos - c) - r;
    float y = (abs(t - 0.5) - 0.5) * length(ab);
    float e = length(max(vec2(x, y), 0.0));
    float i = min(max(x, y), 0.0);

    return e + i;
}

float torus(vec3 pos, vec2 r) {
    float x = length(pos.xz) - r.x;
    float dist = length(vec2(x, pos.y)) - r.y;
    return dist;
}

float box(vec3 pos, vec3 size) {
    pos = abs(pos) - size;
    float dist = length(max(pos, 0.0)) + min(max(pos.x, max(pos.y, pos.z)), 0.0);
    return dist;
}

float get_dist_to_scene(vec3 pos) {
    float plane_dist = pos.y;

    vec3 box_pos = pos;
    box_pos -= vec3(0.0, 1.0, 6.0);
    box_pos.xz *= rotate(world.time);
    float box_dist = box(box_pos, vec3(1.0));

    float sphere_dist = sphere(pos, vec3(0.0, 1.0, 6.0), 1.1);

    float morph_dist = mix(box_dist, sphere_dist, sin(world.time) * 0.5 + 0.5);
    
    float dist = min(morph_dist, plane_dist);

    return dist;
}

float ray_march(vec3 ray_orig, vec3 ray_dir) {
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
    vec2 e = vec2(0.001, 0.0);

    vec3 normal = dist - vec3(
        get_dist_to_scene(pos - e.xyy),
        get_dist_to_scene(pos - e.yxy),
        get_dist_to_scene(pos - e.yyx));

    return normalize(normal);
}

float get_light(vec3 pos) {
    vec3 light_pos = vec3(5.0, 5.0, 8.0);
    vec3 light_dir = normalize(light_pos - pos);
    vec3 normal = get_normal(pos);

    float light = clamp(dot(normal, light_dir) * 0.5 + 0.5, 0.0, 1.0);
    float dist = ray_march(pos + normal * SURF_DIST * 2.0, light_dir);
    if (pos.y < 0.01 && dist < length(light_pos - pos)) light *= 0.5;
    return light;
}

void fragment() {
    vec2 uv = in_uv * 2.0 - 1.0;
    uv.y *= -1.0;

    vec3 ray_orig = vec3(0.0, 4.0, 0.0);
    vec3 ray_dir = normalize(vec3(uv.x, uv.y - 0.4, 1.0));

    vec3 color = vec3(0.0);

    float dist = ray_march(ray_orig, ray_dir);
    vec3 pos = ray_orig + ray_dir * dist;
    float light = get_light(pos);

    color = vec3(light);

    out_color = vec4(color, 1.0);
}
#endif