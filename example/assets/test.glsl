#define PHONG

#ifdef VERTEX
void vertex() {
    gl_Position = world.cam_mat * object.model_mat * vec4(in_position, 1.0);
    out_position = vec3(object.model_mat * vec4(in_position, 1.0));
    out_ls_position = world.light_mat * vec4(out_position, 1.0);
    out_normal = mat3(transpose(inverse(object.model_mat))) * in_normal;
    out_uv = in_uv;
}
#endif

#ifdef FRAGMENT
float hex_dist(vec2 pos) {
    pos = abs(pos);

    float slope_dist = dot(pos, normalize(vec2(1.0, 1.73)));
    float side_dist = pos.x;
    float dist = max(slope_dist, side_dist);

    return dist;
}

vec4 hec_coords(vec2 uv) {
    vec2 r = vec2(1.0, 1.73);
    vec2 h = r * 0.5;
    vec2 a = mod(uv, r) - h;
    vec2 b = mod(uv - h, r) - h;

    vec2 gv;
    if (length(a) < length(b)) {
        gv = a;
    } else {
        gv = b;
    }

    float x = atan(gv.x, gv.y);
    float y = 0.5 - hex_dist(gv);
    vec2 id = uv - gv;
    return vec4(x, y, id.x, id.y);
}

void fragment() {
    vec2 uv = in_uv * 2.0 - 1.0;

    vec3 col = vec3(0);

    uv *= 5.0;

    vec4 hc = hec_coords(uv);

    float c = smoothstep(0.05, 0.1, hc.y * sin(hc.z * hc.w + world.time));

    col += c;

    out_color = vec4(col, 1.0) * phong();
}
#endif