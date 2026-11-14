@group(0) @binding(0) var<uniform> origin: vec4f;
@vertex fn shadow_vertex(@location(0) p: vec3f) -> @builtin(position) vec4f {
    return vec4f(light_position(p - origin.xyz), 1.0);
}
