//! Commit one terrain payload to physics and decal caches together.
use crate::Scene;
use xetal_world_surface::Mesh;
pub(crate) struct SpellUpdates {
    pub sweep: xetal_world_sweep::Update,
    pub bloom: xetal_world_bloom::Update,
    pub ribbon: xetal_world_ribbon::Update,
}
impl Scene {
    pub fn ribbon_input(&mut self, pressed: bool) -> Result<(), String> {
        let pose = self.ribbon_pose();
        if pressed {
            self.ribbon.press(pose)
        } else {
            self.ribbon.release(pose)
        }
    }
    fn ribbon_pose(&self) -> xetal_world_ribbon::Pose {
        xetal_world_ribbon::Pose {
            origin: self.walker.position,
            aim: [self.walker.yaw, self.walker.pitch],
        }
    }
    pub(crate) fn advance_spells(&mut self, dt: f32) -> Result<SpellUpdates, String> {
        let sweep = self.sweep.tick(dt)?;
        if let Some(brush) = sweep.brush {
            self.world.sweep(brush)?;
        }
        let bloom = self.bloom.tick(dt)?;
        if let Some(crater) = bloom.crater {
            self.world.bloom(crater)?;
        }
        let pose = self.ribbon_pose();
        let ribbon = self.ribbon.update(pose, dt, |point| {
            xetal_world_navigation::floor(&self.terrain, point).map(|y| y - 1.7)
        })?;
        if let Some(score) = ribbon.score {
            self.world.ribbon(score)?;
        }
        Ok(SpellUpdates {
            sweep,
            bloom,
            ribbon,
        })
    }
    pub fn cast_bloom(&mut self) -> Result<(), String> {
        let walker = &self.walker;
        let [limit, fallback] = self.bloom.targeting();
        let (s, c) = walker.yaw.sin_cos();
        let (up, flat) = walker.pitch.sin_cos();
        let direction = [s * flat, up, -c * flat];
        let target =
            xetal_world_navigation::raycast(&self.terrain, walker.position, direction, limit)
                .or_else(|| {
                    let point = [
                        walker.position[0] + s * fallback,
                        0.0,
                        walker.position[2] - c * fallback,
                    ];
                    xetal_world_navigation::floor(&self.terrain, point)
                        .map(|y| [point[0], y - 1.7, point[2]])
                })
                .ok_or("Bloom target is outside loaded terrain")?;
        self.bloom.cast(target)
    }
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
