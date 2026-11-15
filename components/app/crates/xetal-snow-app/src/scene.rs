//! Browser physics and capability resources for one X_eTaL application.
use std::collections::BTreeMap;
use xetal_app_session::{Frame, Session};
use xetal_app_tiles::Tiles;
use xetal_world_navigation::Walker;
use xetal_world_surface::Mesh;
use xetal_world_trail::Trail;
pub struct Scene {
    pub app: Session,
    pub world: Tiles,
    pub walker: Walker,
    pub trail: Trail,
    pub frame: Frame,
    pub mesh_hash: u32,
    pub steps: u64,
    pub landings: u64,
    pub wind: f32,
    pub(crate) terrain: Vec<f32>,
    pub(crate) uploaded: BTreeMap<String, u64>,
    pub(crate) audio_sequence: u64,
    pub(crate) particles_revision: u64,
    pub(crate) track_revision: u64,
    pub(crate) jump_sequence: f64,
    pub(crate) reset_sequence: f64,
    pub(crate) elapsed: f32,
    pub(crate) last_pose: Option<xetal_app_session::Pose>,
}
pub struct Update {
    pub terrain: Option<Mesh>,
    pub tracks: Option<Vec<f32>>,
    pub resources: Vec<xetal_app_session::Resource>,
    pub particles: Option<Vec<f32>>,
    pub moving: bool,
    pub sound: [f32; 3],
}
impl Scene {
    pub fn new() -> Result<Self, String> {
        let mut app = Session::new()?;
        let frame = app.read()?;
        Ok(Self {
            app,
            frame,
            world: Tiles::default(),
            walker: Walker::default(),
            trail: Trail::default(),
            terrain: Vec::new(),
            uploaded: BTreeMap::new(),
            mesh_hash: 0,
            steps: 0,
            landings: 0,
            wind: 0.3,
            audio_sequence: 0,
            particles_revision: u64::MAX,
            track_revision: u64::MAX,
            jump_sequence: 0.0,
            reset_sequence: 0.0,
            elapsed: 0.0,
            last_pose: None,
        })
    }
    pub fn input(&mut self, key: &str, pressed: bool) -> Result<(), String> {
        self.app.pose(self.pose())?;
        self.app.input(key, pressed)?;
        self.services()?;
        self.frame = self.app.read()?;
        Ok(())
    }
    pub fn update(&mut self, dt: f32, suspended: bool) -> Result<Update, String> {
        let dt = dt.clamp(0.0, 0.1);
        self.elapsed += if suspended { 0.0 } else { dt };
        if self.elapsed >= 0.05 {
            self.app.tick(self.elapsed)?;
            self.elapsed = 0.0;
        }
        self.services()?;
        let terrain = self.world.step(&mut self.app, self.walker.position)?;
        self.accept_terrain(terrain.as_ref());
        let moving = self.motion(dt, suspended)?;
        self.services()?;
        self.frame = self.app.read()?;
        self.feedback()?;
        let resources = self
            .frame
            .resources
            .iter()
            .filter(|r| self.uploaded.get(&r.id) != Some(&r.revision))
            .cloned()
            .collect::<Vec<_>>();
        for r in &resources {
            self.uploaded.insert(r.id.clone(), r.revision);
        }
        let particles = (self.particles_revision != self.frame.particles_revision)
            .then(|| self.frame.particles.clone());
        self.particles_revision = self.frame.particles_revision;
        let tracks = (self.track_revision != self.trail.revision).then(|| self.trail.vertices());
        self.track_revision = self.trail.revision;
        let sound = if self.audio_sequence != self.frame.audio_sequence {
            self.frame.audio
        } else {
            [0.0, 0.0, 1.0]
        };
        self.audio_sequence = self.frame.audio_sequence;
        Ok(Update {
            terrain,
            tracks,
            resources,
            particles,
            moving,
            sound,
        })
    }
    pub(crate) fn pose(&self) -> xetal_app_session::Pose {
        xetal_app_session::Pose {
            position: self.walker.position,
            aim: [self.walker.yaw, self.walker.pitch],
            grounded: self.walker.motion.grounded,
            impact: self.walker.landing_speed,
        }
    }
    pub(crate) fn feedback(&mut self) -> Result<(), String> {
        self.trail
            .replace(self.frame.prints.as_chunks::<6>().0, &self.terrain)?;
        self.steps = self.frame.stats.get("steps").copied().unwrap_or(0.0) as u64;
        self.landings = self.frame.stats.get("landings").copied().unwrap_or(0.0) as u64;
        self.wind = self.frame.stats.get("wind").copied().unwrap_or(0.0) as f32;
        Ok(())
    }
}
