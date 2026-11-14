struct Camera { angle: f32, tilt: f32, aspect: f32, pad: f32 }
@group(0) @binding(0) var<uniform> camera: Camera;
struct Vertex { @builtin(position) clip: vec4f, @location(0) world: vec3f }
@vertex fn vertex(@location(0) p: vec3f) -> Vertex {
    let c = cos(camera.angle); let s = sin(camera.angle);
    let r = vec3f(c * p.x - s * p.z, p.y, s * p.x + c * p.z);
    let t = cos(camera.tilt); let u = sin(camera.tilt);
    let v = vec3f(r.x, t * r.y - u * r.z, u * r.y + t * r.z);
    let depth = 3.4 - v.z;
    var out: Vertex;
    out.clip = vec4f(1.9 * v.x / camera.aspect, 1.9 * v.y, 1.005 * depth - 0.1005, depth);
    out.world = p;
    return out;
}
@fragment fn fragment(v: Vertex) -> @location(0) vec4f {
    let n = normalize(cross(dpdx(v.world), dpdy(v.world)));
    let light = 0.35 + 0.65 * abs(dot(n, normalize(vec3f(0.4, 1.0, 0.3))));
    let low = vec3f(0.22, 0.48, 0.68);
    let high = vec3f(0.91, 0.97, 1.0);
    let snow = mix(low, high, clamp(0.65 + v.world.y, 0.0, 1.0));
    return vec4f(snow * light, 1.0);
}
