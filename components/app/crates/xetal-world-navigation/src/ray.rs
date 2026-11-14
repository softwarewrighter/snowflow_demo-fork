//! Nearest triangle hit: a physics query, independent of casting policy.
pub fn raycast(
    mesh: &[f32],
    origin: [f32; 3],
    direction: [f32; 3],
    limit: f32,
) -> Option<[f32; 3]> {
    if !limit.is_finite() || limit <= 0.0 || origin.iter().chain(&direction).any(|v| !v.is_finite())
    {
        return None;
    }
    let length = dot(direction, direction).sqrt();
    if length < 1e-6 {
        return None;
    }
    let direction = direction.map(|v| v / length);
    let distance = mesh
        .as_chunks::<9>()
        .0
        .iter()
        .filter_map(|triangle| hit(triangle, origin, direction))
        .filter(|t| *t <= limit)
        .min_by(f32::total_cmp)?;
    Some(std::array::from_fn(|i| origin[i] + direction[i] * distance))
}
fn hit(p: &[f32; 9], origin: [f32; 3], direction: [f32; 3]) -> Option<f32> {
    let a = std::array::from_fn(|i| p[i + 3] - p[i]);
    let b = std::array::from_fn(|i| p[i + 6] - p[i]);
    let h = cross(direction, b);
    let determinant = dot(a, h);
    if determinant.abs() < 1e-7 {
        return None;
    }
    let s = std::array::from_fn(|i| origin[i] - p[i]);
    let u = dot(s, h) / determinant;
    let q = cross(s, a);
    let v = dot(direction, q) / determinant;
    let t = dot(b, q) / determinant;
    (u >= 0.0 && v >= 0.0 && u + v <= 1.0 && t > 0.0001).then_some(t)
}
fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
