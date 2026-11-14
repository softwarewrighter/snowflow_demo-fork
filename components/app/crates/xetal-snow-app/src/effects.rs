//! Commit one terrain payload to physics and decal caches together.
use crate::Scene;
use xetal_world_surface::Mesh;
impl Scene {
    pub(crate) fn accept_terrain(&mut self, mesh: Option<&Mesh>) {
        if let Some(mesh) = mesh {
            self.terrain = mesh.positions().to_vec();
            self.mesh_hash = mesh.positions().iter().fold(2166136261_u32, |hash, value| {
                (hash ^ value.to_bits()).wrapping_mul(16777619)
            });
        }
        if self.terrain_revision != self.world.edit_revision {
            self.trail.redrape(&self.terrain);
            self.terrain_revision = self.world.edit_revision;
        }
    }
}
