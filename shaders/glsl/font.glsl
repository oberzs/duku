// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// bitmap font shader

#define DEPTH write
#define CULL back
#define SHAPE filled_triangles 

layout(location = 0) out vec4 out_color;

void fragment() {
    vec3 tint = material.arg_1.rgb;
    float alpha = tex(0, in_uv).r;

    out_color = vec4(tint, alpha);
}
