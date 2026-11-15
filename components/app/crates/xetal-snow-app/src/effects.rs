//! Generic physics queries, movement facts and committed terrain resources.
use crate::Scene;
use xetal_world_navigation::{Walker, floor, raycast};
use xetal_world_surface::Mesh;
impl Scene {
    pub(crate) fn services(&mut self) -> Result<(), String> {
        for _ in 0..16 {
            let frame = self.app.read()?;
            let Some(q) = frame.requests.as_chunks::<10>().0.first() else {
                return Ok(());
            };
            let origin = [q[2], q[3], q[4]];
            let point = match q[1] {
                1.0 => raycast(&self.terrain, origin, [q[5], q[6], q[7]], q[8]).or_else(|| {
                    let length = q[5].hypot(q[7]).max(0.0001);
                    let p = [
                        origin[0] + q[5] * q[9] / length,
                        0.0,
                        origin[2] + q[7] * q[9] / length,
                    ];
                    floor(&self.terrain, p).map(|y| [p[0], y - 1.7, p[2]])
                }),
                2.0 => floor(&self.terrain, origin).map(|y| [origin[0], y - 1.7, origin[2]]),
                _ => return Err("unsupported physics capability".into()),
            };
            self.app.respond(q[0] as u32, point)?;
        }
        Err("application query budget exceeded".into())
    }
    pub(crate) fn motion(&mut self, dt: f32, suspended: bool) -> Result<bool, String> {
        let frame = self.app.read()?;
        let nav = frame.navigation;
        if nav[6] != self.reset_sequence {
            self.walker = Walker::default();
            self.reset_sequence = nav[6];
            self.jump_sequence = nav[3];
        }
        self.walker.yaw = nav[4] as f32;
        self.walker.pitch = nav[5] as f32;
        let before = self.walker.position;
        let paused = suspended || nav[0] != 0.0;
        if !paused {
            self.walker.advance(dt, nav[1] as f32, nav[2] as f32);
            if floor(&self.terrain, self.walker.position).is_none() {
                self.walker.position = before;
            }
        }
        self.walker.simulate(
            if paused { 0.0 } else { dt },
            !paused && nav[3] != self.jump_sequence,
            &self.terrain,
        );
        self.jump_sequence = nav[3];
        self.app.pose(self.pose())?;
        Ok(!paused
            && self.walker.motion.grounded
            && (before[0] - self.walker.position[0]).hypot(before[2] - self.walker.position[2])
                > 0.0001)
    }
    pub(crate) fn accept_terrain(&mut self, mesh: Option<&Mesh>) {
        if let Some(mesh) = mesh {
            self.terrain = mesh.positions().to_vec();
            self.mesh_hash = self.terrain.iter().fold(2166136261_u32, |hash, value| {
                (hash ^ value.to_bits()).wrapping_mul(16777619)
            });
            self.trail.redrape(&self.terrain);
        }
    }
}
