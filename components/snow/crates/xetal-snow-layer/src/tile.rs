use crate::{
    plan::Key,
    sampling::{self, Coarse},
};
pub struct Tile {
    pub key: Key,
    pub spacing: f32,
    pub hash: u64,
    pub rows: Vec<f32>,
    pub heights: Vec<f32>,
    pub n: usize,
}
impl Tile {
    pub fn new(key: Key, spacing: f32, hash: u64, rows: Vec<f32>) -> Result<Self, String> {
        let n = (2.0 / spacing).round() as usize + 1;
        if rows.len() != n * n * 3
            || rows.iter().any(|n| !n.is_finite())
            || rows
                .as_chunks::<3>()
                .0
                .iter()
                .any(|p| p[1] < -0.076 || p[1] > 0.026)
        {
            return Err("invalid fine grid payload".into());
        }
        for (i, p) in rows.as_chunks::<3>().0.iter().enumerate() {
            let x = key.0 as f32 * 2.0 + (i / n) as f32 * spacing;
            let z = key.1 as f32 * 2.0 + (i % n) as f32 * spacing;
            if (p[0] - x).abs() > 0.0001 || (p[2] - z).abs() > 0.0001 {
                return Err("invalid grid lattice".into());
            }
        }
        Ok(Self {
            key,
            spacing,
            hash,
            rows,
            heights: Vec::new(),
            n,
        })
    }
    pub fn project(&self, coarse: &Coarse, keys: &std::collections::BTreeSet<Key>) -> Self {
        let heights = self
            .rows
            .as_chunks::<3>()
            .0
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let x = i / self.n;
                let z = i % self.n;
                let dx = if x == 0 {
                    -1
                } else if x == self.n - 1 {
                    1
                } else {
                    0
                };
                let dz = if z == 0 {
                    -1
                } else if z == self.n - 1 {
                    1
                } else {
                    0
                };
                let edge = [(dx, 0), (0, dz), (dx, dz)]
                    .iter()
                    .any(|(x, z)| !keys.contains(&(self.key.0 + x, self.key.1 + z)));
                let delta = if edge { 0.0 } else { p[1] };
                coarse.floor(*p).map_or(f32::NAN, |y| y - 1.7 + delta)
            })
            .collect();
        Self {
            key: self.key,
            spacing: self.spacing,
            hash: self.hash,
            rows: self.rows.clone(),
            heights,
            n: self.n,
        }
    }
    pub fn floor(&self, p: [f32; 3]) -> Option<f32> {
        if self.heights.len() != self.n * self.n || self.heights.iter().any(|n| !n.is_finite()) {
            return None;
        }
        sampling::height(
            &self.heights,
            self.n,
            (p[0] - self.key.0 as f32 * 2.0) / self.spacing,
            (p[2] - self.key.1 as f32 * 2.0) / self.spacing,
        )
        .filter(|y| y.is_finite())
        .map(|y| y + 1.7)
    }
}
