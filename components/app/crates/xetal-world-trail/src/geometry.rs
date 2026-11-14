use xetal_world_navigation::floor;

pub(crate) fn quad(position: [f32; 3], yaw: f32, terrain: &[f32]) -> Option<[f32; 30]> {
    let corners: Option<Vec<_>> = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
        .iter()
        .map(|&(u, v)| corner(position, yaw, u, v, terrain))
        .collect();
    let corners = corners?;
    let mut vertices = [0.0; 30];
    for (i, corner) in [0, 1, 2, 0, 2, 3].into_iter().enumerate() {
        vertices[i * 5..i * 5 + 5].copy_from_slice(&corners[corner]);
    }
    Some(vertices)
}

fn corner(position: [f32; 3], yaw: f32, u: f32, v: f32, terrain: &[f32]) -> Option<[f32; 5]> {
    let (s, c) = yaw.sin_cos();
    let x = position[0] + 0.12 * u * c + 0.23 * v * s;
    let z = position[2] + 0.12 * u * s - 0.23 * v * c;
    let y = floor(terrain, [x, 0.0, z])? - 1.7 + 0.012;
    Some([x, y, z, u, v])
}
