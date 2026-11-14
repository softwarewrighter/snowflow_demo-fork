struct FlakeVertex {
    @builtin(position) clip: vec4f,
    @location(0) uv: vec2f,
    @location(1) alpha: f32,
}
@vertex fn particle_vertex(@builtin(vertex_index) vertex: u32,
    @location(0) center: vec3f, @location(1) size: f32,
    @location(2) alpha: f32, @location(3) spin: f32) -> FlakeVertex {
    let x = select(-1.0, 1.0, vertex == 1u || vertex == 2u || vertex == 4u);
    let y = select(-1.0, 1.0, vertex == 2u || vertex == 4u || vertex == 5u);
    let uv = vec2f(x, y);
    let rotated = vec2f(cos(spin) * x - sin(spin) * y, sin(spin) * x + cos(spin) * y);
    var out: FlakeVertex;
    out.clip = project(center);
    out.clip.x += rotated.x * size * 1.9 / camera.aspect;
    out.clip.y += rotated.y * size * 1.9;
    out.uv = uv;
    out.alpha = alpha;
    return out;
}
@fragment fn particle_fragment(v: FlakeVertex) -> @location(0) vec4f {
    let r = length(v.uv);
    let sector = round(atan2(v.uv.y, v.uv.x) / 1.0471975512) * 1.0471975512;
    let p = vec2f(cos(sector) * v.uv.x + sin(sector) * v.uv.y,
        -sin(sector) * v.uv.x + cos(sector) * v.uv.y);
    let stem = 1.0 - smoothstep(0.025, 0.07, abs(p.y));
    let branches = (1.0 - smoothstep(0.025, 0.07, abs(abs(p.y) - 0.6 * (0.65 - p.x))))
        * smoothstep(0.25, 0.4, p.x) * (1.0 - smoothstep(0.65, 0.75, p.x));
    let mask = max(stem, branches) * (1.0 - smoothstep(0.85, 1.0, r));
    let alpha = mask * v.alpha * 0.85;
    if alpha < 0.01 { discard; }
    return vec4f(0.92, 0.97, 1.0, alpha);
}
