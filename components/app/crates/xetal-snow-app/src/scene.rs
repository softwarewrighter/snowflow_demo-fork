//! Native-testable streaming, navigation and grounded-footfall coordination.
use xetal_world_model::{Player, Pose};
use xetal_world_navigation::{Walker, floor};
use xetal_world_stream::World;
use xetal_world_trail::Trail;

pub struct Scene {
    pub world: World,
    pub walker: Walker,
    pub trail: Trail,
    pub mesh_hash: u32,
    pub landings: u64,
    pub steps: u64,
    pub wind: f32,
    player: Player,
    terrain: Vec<f32>,
    track_revision: u64,
}
pub struct Update {
    pub terrain: Option<xetal_world_surface::Mesh>,
    pub tracks: Option<Vec<f32>>,
    pub step: bool,
    pub landing: f32,
    pub moving: bool,
    pub sound: [f32;3],
}
impl Scene {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            world: World::new()?,
            walker: Walker::default(),
            trail: Trail::default(),
            terrain: Vec::new(),
            mesh_hash: 0,
            landings: 0,
            steps: 0, wind: 0.3, player: Player::new()?,
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
        let (pose, moving) = self.motion(dt, held, jump, paused);
        let mut sound = [0.0,0.0,1.0];
        if let Some(commands) = self.player.update(pose, if paused { 0.0 } else { dt })? {
            self.trail.replace(&commands.prints, &self.terrain)?;
            self.steps = commands.steps; self.landings = commands.landings; self.wind = commands.wind; sound = commands.sound;
        }
        let step = sound[0] == 1.0;
        let landing = if sound[0] == 2.0 { sound[1] } else { 0.0 };
        let tracks = (self.track_revision != self.trail.revision).then(|| self.trail.vertices());
        self.track_revision = self.trail.revision;
        Ok(Update {
            terrain,
            tracks,
            step,
            landing,
            moving, sound,
        })
    }
    fn motion(&mut self, dt: f32, held: [bool; 4], jump: bool, paused: bool) -> (Pose, bool) {
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
        let moving = !paused
            && self.walker.motion.grounded
            && (before[0] - self.walker.position[0]).hypot(before[2] - self.walker.position[2])
                > 0.0001;
        (Pose { point: [self.walker.position[0], self.walker.position[2]], yaw: self.walker.yaw, grounded: self.walker.motion.grounded, impact: self.walker.landing_speed }, moving)
    }
    pub fn reset(&mut self) -> Result<(), String> { self.walker = Walker::default(); self.clear_tracks() }
    pub fn clear_tracks(&mut self) -> Result<(), String> {
        self.player.reset(Pose { point: [self.walker.position[0],self.walker.position[2]], yaw: self.walker.yaw, grounded: self.walker.motion.grounded, impact: 0.0 })?;
        self.trail = Trail::default(); self.steps = 0; self.landings = 0; self.track_revision = u64::MAX;
        Ok(())
    }
}
