//! GPU representation cache only: X_eTaL supplies placement, history and opacity.
use crate::geometry;
use std::collections::{BTreeMap, VecDeque};

pub struct Mark {
    pub position: [f32; 3],
    pub vertices: [f32; 30],
    pub opacity: f32,
    key: [u32; 5],
}
#[derive(Default)]
pub struct Trail {
    pub marks: VecDeque<Mark>,
    pub revision: u64,
}
impl Trail {
    pub fn replace(&mut self, rows: &[[f32; 6]], terrain: &[f32]) -> Result<(), String> {
        if rows.len() > 256 {
            return Err("footprint resource budget exceeded".into());
        }
        if self.marks.len() == rows.len()
            && self
                .marks
                .iter()
                .zip(rows)
                .all(|(mark, row)| mark.key == key(row) && mark.opacity == row[5])
        {
            return Ok(());
        }
        let mut cache: BTreeMap<_, _> = self.marks.drain(..).map(|mark| (mark.key, mark)).collect();
        for row in rows {
            let key = key(row);
            let mark = cache.remove(&key).or_else(|| create(row, terrain));
            if let Some(mut mark) = mark {
                mark.opacity = row[5].clamp(0.0, 1.0);
                self.marks.push_back(mark);
            }
        }
        self.revision += 1;
        Ok(())
    }
    /// Reproject cached decals after a committed terrain edit.
    pub fn redrape(&mut self, terrain: &[f32]) {
        for mark in &mut self.marks {
            if let Some(vertices) =
                geometry::quad(mark.position, f32::from_bits(mark.key[2]), terrain)
            {
                mark.vertices = vertices;
            }
        }
        self.revision += 1;
    }
    pub fn vertices(&self) -> Vec<f32> {
        let mut vertices = Vec::with_capacity(self.marks.len() * 36);
        for mark in self.marks.iter().filter(|mark| mark.opacity > 0.0) {
            for vertex in mark.vertices.as_chunks::<5>().0 {
                vertices.extend_from_slice(vertex);
                vertices.push(mark.opacity);
            }
        }
        vertices
    }
}
fn key(row: &[f32; 6]) -> [u32; 5] {
    [
        row[0].to_bits(),
        row[1].to_bits(),
        row[2].to_bits(),
        row[3].to_bits(),
        row[4].to_bits(),
    ]
}
fn create(row: &[f32; 6], terrain: &[f32]) -> Option<Mark> {
    let (s, c) = row[2].sin_cos();
    let position = [row[0] + 0.12 * c * row[3], 0.0, row[1] + 0.12 * s * row[3]];
    Some(Mark {
        position,
        vertices: geometry::quad(position, row[2], terrain)?,
        opacity: row[5],
        key: key(row),
    })
}
