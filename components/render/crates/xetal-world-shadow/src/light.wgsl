// Shared by the depth and visible passes to keep projections identical.
fn light_position(p: vec3f) -> vec3f {
    let right = vec3f(0.6, 0.0, -0.8);
    let up = vec3f(-0.71554175, 0.44721360, -0.53665631);
    let direction = vec3f(0.35777088, 0.89442719, 0.26832816);
    return vec3f(dot(p, right) / 24.0, dot(p, up) / 24.0, (40.0 - dot(p, direction)) / 80.0);
}
