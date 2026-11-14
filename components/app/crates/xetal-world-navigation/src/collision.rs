pub fn floor(positions: &[f32], position: [f32; 3]) -> Option<f32> {
    positions
        .as_chunks::<9>()
        .0
        .iter()
        .find_map(|triangle| height(triangle, position[0], position[2]).map(|y| y + 1.7))
}

fn height(p: &[f32], x: f32, z: f32) -> Option<f32> {
    let (ax, az, bx, bz) = (p[3] - p[0], p[5] - p[2], p[6] - p[0], p[8] - p[2]);
    let det = ax * bz - az * bx;
    if det.abs() < 1e-8 {
        return None;
    }
    let u = ((x - p[0]) * bz - (z - p[2]) * bx) / det;
    let v = (ax * (z - p[2]) - az * (x - p[0])) / det;
    (u >= -1e-5 && v >= -1e-5 && u + v <= 1.00001)
        .then(|| p[1] + u * (p[4] - p[1]) + v * (p[7] - p[1]))
}
