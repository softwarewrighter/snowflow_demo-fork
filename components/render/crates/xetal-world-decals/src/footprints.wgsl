struct Print { @builtin(position) clip: vec4f, @location(0) uv: vec2f, @location(1) world: vec3f }
@vertex fn print_vertex(@location(0) p: vec3f, @location(1) uv: vec2f) -> Print {
    var out: Print; out.clip = project(p); out.uv = uv; out.world = p; return out;
}
@fragment fn print_fragment(v: Print) -> @location(0) vec4f {
    let width = 0.65 + 0.25 * smoothstep(-1.0, 1.0, v.uv.y);
    let shape = dot(vec2f(v.uv.x / width, v.uv.y), vec2f(v.uv.x / width, v.uv.y));
    let grooves = 0.85 + 0.15 * step(0.5, fract((v.uv.y + 1.0) * 8.0));
    let haze = 1.0 - smoothstep(40.0, 60.0, distance(camera.eye.xz, v.world.xz));
    let alpha = (1.0 - smoothstep(0.72, 1.0, shape)) * 0.42 * grooves * haze;
    return vec4f(0.28, 0.43, 0.52, alpha);
}
