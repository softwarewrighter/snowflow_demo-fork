use crate::{Key, center, merge, ring};
use std::collections::BTreeMap;
use xetal_world_model::{Demo, Progress};
use xetal_world_surface::Mesh;

pub struct World {
    pub(crate) demo: Demo,
    pub(crate) chunks: BTreeMap<Key, Mesh>,
    requested: Option<Key>,
    pub(crate) pending_edit: Option<[f32; 5]>,
    pub(crate) pending_reset: bool,
    pub edit_revision: u64,
    pub center: Key,
    pub generated: u64,
}

impl World {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            demo: Demo::from_source(crate::SOURCE)?,
            pending_edit: None,
            pending_reset: false,
            edit_revision: 0,
            chunks: BTreeMap::new(),
            requested: Some((0, 0)),
            center: (0, 0),
            generated: 0,
        })
    }

    pub fn step(&mut self, position: [f32; 3], budget: usize) -> Result<Option<Mesh>, String> {
        if self.requested.is_none() && (self.pending_reset || self.pending_edit.is_some()) {
            return self.apply_edits();
        }
        let next = center(position);
        if next == self.center && self.requested.is_none() && self.chunks.len() == 25 {
            return Ok(None);
        }
        self.center = next;
        let wanted = ring(self.center);
        let previous = self.chunks.len();
        self.chunks.retain(|key, _| wanted.contains(key));
        let mut changed = previous != self.chunks.len();
        if self.requested.is_none()
            && let Some(key) = wanted.iter().find(|key| !self.chunks.contains_key(key))
        {
            self.demo.event(&format!("choose {} {}", key.0, key.1))?;
            self.requested = Some(*key);
        }
        if let Some(key) = self.requested
            && self.demo.step(budget)? == Progress::Waiting
        {
            if wanted.contains(&key) {
                self.chunks.insert(key, self.demo.mesh()?);
                changed = true;
            }
            self.requested = None;
            self.generated += 1;
        }
        if changed {
            return merge::mesh(&self.chunks).map(Some);
        }
        Ok(None)
    }

    pub fn resident(&self) -> usize {
        self.chunks.len()
    }
}
