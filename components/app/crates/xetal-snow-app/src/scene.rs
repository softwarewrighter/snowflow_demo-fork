//! Native-testable streaming, navigation and grounded-footfall coordination.
use xetal_world_navigation::{Walker, floor};
use xetal_world_stream::World;
use xetal_world_trail::Trail;

pub struct Scene {
    pub world: World,
    pub walker: Walker,
    pub trail: Trail,
    pub mesh_hash: u32,
    pub landings: u64,
    terrain: Vec<f32>,
    track_revision: u64,
}
pub struct Update {
    pub terrain: Option<xetal_world_surface::Mesh>,
    pub tracks: Option<Vec<f32>>,
    pub step: bool,
    pub landing: f32,
    pub moving: bool,
}
impl Scene {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            world: World::new()?,
            walker: Walker::default(),
            trail: Trail::new([0.0, 1.7, 8.0]),
            terrain: Vec::new(),
            mesh_hash: 0,
            landings: 0,
            track_revision: u64::MAX,
        })
    }
    pub fn update(
        &mut self,
        dt: f32,
        held: [bool; 4],
        jump: bool,
        paused: bool,
    ) -> Result<Update, String> {
        let terrain = self.world.step(self.walker.position, 2_000)?;
        if let Some(mesh) = &terrain {
            self.terrain = mesh.positions().to_vec();
            self.mesh_hash = mesh.positions().iter().fold(2166136261_u32, |hash, value| {
                (hash ^ value.to_bits()).wrapping_mul(16777619)
            });
        }
        let (step, landing, moving) = self.motion(dt, held, jump, paused);
        let tracks = (self.track_revision != self.trail.revision).then(|| self.trail.vertices());
        self.track_revision = self.trail.revision;
        Ok(Update {
            terrain,
            tracks,
            step,
            landing,
            moving,
        })
    }
    fn motion(&mut self, dt: f32, held: [bool; 4], jump: bool, paused: bool) -> (bool, f32, bool) {
        let before = self.walker.position;
        if !paused {
            self.walker.advance(
                dt,
                (i32::from(held[0]) - i32::from(held[1])) as f32,
                (i32::from(held[3]) - i32::from(held[2])) as f32,
            );
            if floor(&self.terrain, self.walker.position).is_none() {
                self.walker.position = before;
            }
        }
        self.walker.simulate(
            if paused { 0.0 } else { dt },
            jump && !paused,
            &self.terrain,
        );
        let landing = self.walker.landing_speed;
        if landing > 0.0 {
            self.landings += 1;
        }
        let step = self.trail.update(
            self.walker.position,
            self.walker.yaw,
            self.walker.motion.grounded,
            landing,
            &self.terrain,
        );
        let moving = !paused
            && self.walker.motion.grounded
            && (before[0] - self.walker.position[0]).hypot(before[2] - self.walker.position[2])
                > 0.0001;
        (step, landing, moving)
    }
    pub fn reset(&mut self) {
        self.walker = Walker::default();
        self.landings = 0;
        self.clear_tracks();
    }
    pub fn clear_tracks(&mut self) {
        self.trail = Trail::new(self.walker.position);
        self.track_revision = u64::MAX;
    }
}
