struct Camera { angle: f32, tilt: f32, aspect: f32, pad: f32, eye: vec4f }
@group(0) @binding(0) var<uniform> camera: Camera;
@group(1) @binding(0) var sun_depth: texture_depth_2d;
@group(1) @binding(1) var sun_sampler: sampler_comparison;

fn grain(p: vec3f) -> f32 {
    var q = fract(p * vec3f(0.1031, 0.1030, 0.0973));
    q += dot(q, q.yzx + 33.33);
    return fract((q.x + q.y) * q.z);
}

fn visibility(p: vec3f, n: vec3f) -> f32 {
    if camera.eye.w < 0.5 { return 1.0; }
    let light = light_position(p);
    let uv = vec2f(0.5 + 0.5 * light.x, 0.5 - 0.5 * light.y);
    if any(uv < vec2f(0.0)) || any(uv > vec2f(1.0)) { return 1.0; }
    let bias = 0.0004 + 0.001 * (1.0 - abs(n.y));
    return textureSampleCompareLevel(sun_depth, sun_sampler, uv, light.z - bias);
}

struct Vertex { @builtin(position) clip: vec4f, @location(0) world: vec3f }
@vertex fn vertex(@location(0) p: vec3f) -> Vertex {
    let q = p - camera.eye.xyz;
    let c = cos(camera.angle); let s = sin(camera.angle);
    let r = vec3f(c * q.x + s * q.z, q.y, -s * q.x + c * q.z);
    let t = cos(camera.tilt); let u = sin(camera.tilt);
    let v = vec3f(r.x, t * r.y + u * r.z, -u * r.y + t * r.z);
    let depth = -v.z;
    var out: Vertex;
    out.clip = vec4f(1.9 * v.x / camera.aspect, 1.9 * v.y, 1.0005 * depth - 0.050025, depth);
    out.world = p;
    return out;
}
@fragment fn fragment(v: Vertex) -> @location(0) vec4f {
    let n = normalize(cross(dpdx(v.world), dpdy(v.world)));
    let light = 0.35 + 0.65 * abs(dot(n, normalize(vec3f(0.4, 1.0, 0.3))));
    let ice = vec3f(0.25, 0.65, 0.82);
    let white = vec3f(0.91, 0.97, 1.0);
    var snow = mix(ice, white, smoothstep(0.45, 0.9, abs(n.y)));
    let footprint = length(fwidth(v.world));
    if camera.pad > 0.5 {
        let fine = mix(grain(floor(v.world * 24.0)), 0.5, smoothstep(0.02, 0.12, footprint));
        let wind = sin(v.world.x * 3.0 + 0.7 * sin(v.world.z * 1.3));
        let cracks = smoothstep(0.0, 0.035, abs(fract(v.world.x * 1.2 + v.world.z * 0.7) - 0.5));
        let coverage = smoothstep(0.45, 0.9, abs(n.y));
        snow *= mix(0.8 + 0.2 * cracks, 0.93 + 0.05 * wind + 0.08 * fine, coverage);
    }
    let shade = 0.45 + 0.55 * visibility(v.world, n);
    let fog = 1.0 - exp(-0.025 * distance(camera.eye.xyz, v.world));
    return vec4f(mix(snow * light * shade, vec3f(0.47, 0.65, 0.78), fog), 1.0);
}
