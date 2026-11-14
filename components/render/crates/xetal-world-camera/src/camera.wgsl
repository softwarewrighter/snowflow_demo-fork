struct Camera { angle: f32, tilt: f32, aspect: f32, pad: f32, eye: vec4f }
@group(0) @binding(0) var<uniform> camera: Camera;
fn project(p: vec3f) -> vec4f {
    let q = p - camera.eye.xyz;
    let c = cos(camera.angle); let s = sin(camera.angle);
    let r = vec3f(c * q.x + s * q.z, q.y, -s * q.x + c * q.z);
    let t = cos(camera.tilt); let u = sin(camera.tilt);
    let v = vec3f(r.x, t * r.y + u * r.z, -u * r.y + t * r.z);
    let depth = -v.z;
    return vec4f(1.9 * v.x / camera.aspect, 1.9 * v.y, 1.0005 * depth - 0.050025, depth);
}
