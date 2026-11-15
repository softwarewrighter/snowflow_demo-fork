use std::collections::BTreeMap;
#[derive(Default)]
pub struct Coarse {
    cells: BTreeMap<(i32, i32), Vec<f32>>,
}
impl Coarse {
    pub fn new(positions: &[f32]) -> Self {
        let mut cells: BTreeMap<_, Vec<f32>> = BTreeMap::new();
        for t in positions.as_chunks::<9>().0 {
            let x = t[0].min(t[3]).min(t[6]);
            let z = t[2].min(t[5]).min(t[8]);
            cells
                .entry(((x / 2.0).floor() as i32, (z / 2.0).floor() as i32))
                .or_default()
                .extend_from_slice(t);
        }
        Self { cells }
    }
    pub fn floor(&self, p: [f32; 3]) -> Option<f32> {
        let key = ((p[0] / 2.0).floor() as i32, (p[2] / 2.0).floor() as i32);
        [(0, 0), (-1, 0), (0, -1), (-1, -1)]
            .into_iter()
            .find_map(|(x, z)| {
                self.cells
                    .get(&(key.0 + x, key.1 + z))
                    .and_then(|rows| xetal_world_navigation::floor(rows, p))
            })
    }
}
pub fn height(heights: &[f32], n: usize, fx: f32, fz: f32) -> Option<f32> {
    let x = (fx.floor() as usize).min(n - 2);
    let z = (fz.floor() as usize).min(n - 2);
    let u = (fx - x as f32).clamp(0.0, 1.0);
    let v = (fz - z as f32).clamp(0.0, 1.0);
    let [a, b, c, d] = [
        heights[x * n + z],
        heights[(x + 1) * n + z],
        heights[x * n + z + 1],
        heights[(x + 1) * n + z + 1],
    ];
    Some(if u + v <= 1.0 {
        a + u * (b - a) + v * (c - a)
    } else {
        d + (1.0 - u) * (c - d) + (1.0 - v) * (b - d)
    })
}
