//! Atomic render/collision snapshots; at most one X_eTaL tile job per update.
use crate::{compose, plan, sampling::Coarse, tile::Tile};
use std::{collections::BTreeMap, sync::Arc};
use xetal_app_session::{Frame, Session};
use xetal_world_surface::Mesh;
#[derive(Default)]
pub struct Layer {
    pub(crate) base: Option<Arc<Mesh>>,
    pub(crate) coarse: Coarse,
    pub(crate) tiles: BTreeMap<plan::Key, Arc<Tile>>,
    pending: Option<plan::Pending>,
    pub revision: u64,
    signature: u64,
}
impl Layer {
    pub fn update(
        &mut self,
        app: &mut Session,
        frame: &Frame,
        position: [f32; 3],
        base: Option<Mesh>,
    ) -> Result<Option<Mesh>, String> {
        let changed = base.is_some();
        if let Some(base) = base {
            self.coarse = Coarse::new(base.positions());
            self.base = Some(Arc::new(base));
        }
        if self.base.is_none() {
            return Ok(None);
        }
        let desired = plan::desired(&frame.detail, position);
        let signature = plan::signature(&desired, frame.detail.config);
        if self
            .pending
            .as_ref()
            .is_some_and(|p| p.signature != signature)
        {
            self.pending = None;
        }
        if signature != self.signature && self.pending.is_none() {
            self.pending = Some(plan::pending(desired, signature, &self.tiles));
        }
        if let Some(p) = self.pending.as_mut() {
            if let Some((key, hash)) = p.jobs.pop() {
                let rows = app.detail_tile(key, frame.detail.config[0])?;
                p.tiles.insert(
                    key,
                    Arc::new(Tile::new(key, frame.detail.config[0], hash, rows)?),
                );
            }
            if p.jobs.is_empty() {
                let p = self.pending.take().ok_or("missing pending field")?;
                return self.commit(p.tiles, signature).map(Some);
            }
        }
        if changed {
            return self.commit(self.tiles.clone(), self.signature).map(Some);
        }
        Ok(None)
    }
    fn commit(
        &mut self,
        mut tiles: BTreeMap<plan::Key, Arc<Tile>>,
        signature: u64,
    ) -> Result<Mesh, String> {
        let mesh = compose::mesh(
            self.base.as_ref().ok_or("missing base")?,
            &mut tiles,
            &self.coarse,
        )?;
        self.tiles = tiles;
        self.signature = signature;
        self.revision += 1;
        Ok(mesh)
    }
    pub fn floor(&self, p: [f32; 3]) -> Option<f32> {
        let key = ((p[0] / 2.0).floor() as i32, (p[2] / 2.0).floor() as i32);
        self.tiles
            .get(&key)
            .and_then(|t| t.floor(p))
            .or_else(|| self.coarse.floor(p))
    }
    pub fn resident(&self) -> usize {
        self.tiles.len()
    }
    pub fn memory_bytes(&self) -> usize {
        self.tiles
            .values()
            .map(|t| t.rows.len() * 4 + t.heights.len() * 4)
            .sum::<usize>()
            + self.base.as_ref().map_or(0, |m| m.byte_len())
    }
}
