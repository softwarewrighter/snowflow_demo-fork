use crate::{Key, center, merge, ring};
use std::{collections::BTreeMap, sync::Arc};
use xetal_app_session::{Frame, Session};
use xetal_world_surface::Mesh;
#[derive(Default)]
pub struct Tiles {
    chunks: BTreeMap<Key, Arc<Mesh>>,
    pub center: Key,
    pub generated: u64,
    pub edit_revision: u64,
}
impl Tiles {
    pub fn resident(&self) -> usize {
        self.chunks.len()
    }
    pub fn step(&mut self, app: &mut Session, position: [f32; 3]) -> Result<Option<Mesh>, String> {
        self.center = center(position);
        let wanted = ring(self.center);
        let frame = app.read()?;
        let mut replacement = BTreeMap::new();
        for (&key, mesh) in &self.chunks {
            if !wanted.contains(&key) {
                continue;
            }
            replacement.insert(
                key,
                if dirty(key, self.edit_revision, &frame) {
                    Arc::new(app.chunk(key)?)
                } else {
                    mesh.clone()
                },
            );
        }
        let mut changed =
            replacement.len() != self.chunks.len() || frame.world_revision != self.edit_revision;
        if let Some(key) = wanted.iter().find(|key| !replacement.contains_key(key)) {
            replacement.insert(*key, Arc::new(app.chunk(*key)?));
            self.generated += 1;
            changed = true;
        }
        if !changed {
            return Ok(None);
        }
        let mesh = merge::mesh(&replacement)?;
        app.acknowledge(frame.world_revision)?;
        self.chunks = replacement;
        self.edit_revision = frame.world_revision;
        Ok(Some(mesh))
    }
}
fn dirty(key: Key, revision: u64, frame: &Frame) -> bool {
    if revision == frame.world_revision {
        return false;
    }
    let x = key.0 as f32 * 32.0;
    let z = key.1 as f32 * 32.0;
    frame
        .dirty
        .as_chunks::<4>()
        .0
        .iter()
        .any(|b| x <= b[1] && x + 32.0 >= b[0] && z <= b[3] && z + 32.0 >= b[2])
}
