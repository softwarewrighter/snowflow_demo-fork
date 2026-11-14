@vertex fn shadow_vertex(@location(0) p: vec3f) -> @builtin(position) vec4f {
    return vec4f(light_position(p), 1.0);
}
