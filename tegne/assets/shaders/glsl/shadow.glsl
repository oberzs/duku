#ifdef VERTEX
void vertex() {
    gl_Position = world.light_mat * object.model_mat * vec4(in_position, 1.0);
}
#endif

#ifdef FRAGMENT
void fragment() {}
#endif