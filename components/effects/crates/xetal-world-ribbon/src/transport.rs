use crate::{
    Pose, SOURCE, Update, io,
    output::{self, Cache},
};
use xetal_world_model::Demo;
pub struct Ribbon {
    pub(crate) demo: Demo,
    pub(crate) cache: Cache,
    pub(crate) last: Option<Pose>,
    pub(crate) elapsed: f32,
    pub(crate) cleared: bool,
}
impl Ribbon {
    pub fn new() -> Result<Self, String> {
        let mut demo = Demo::from_source(SOURCE)?;
        io::settle(&mut demo)?;
        Ok(Self {
            demo,
            cache: Cache::default(),
            last: None,
            elapsed: 0.0,
            cleared: false,
        })
    }
    pub fn update(
        &mut self,
        pose: Pose,
        dt: f32,
        ground: impl Fn([f32; 3]) -> Option<f32>,
    ) -> Result<Update, String> {
        pose.validate()?;
        if !dt.is_finite() || dt < 0.0 {
            return Err("invalid Ribbon tick".into());
        }
        if self.cleared {
            self.cleared = false;
            return Ok(Update {
                mesh: Some(self.demo.mesh()?),
                score: None,
            });
        }
        self.elapsed += dt.min(0.1);
        if self.elapsed < 0.05 {
            return Ok(Update::default());
        }
        self.pose(pose)?;
        io::send(&mut self.demo, &format!("tick {}", self.elapsed))?;
        self.elapsed = 0.0;
        if let Some(probe) = output::probe(&self.demo)?
            && let Some(height) = ground(probe)
        {
            if !height.is_finite() {
                return Err("nonfinite Ribbon ground fact".into());
            }
            io::send(&mut self.demo, &format!("choose {height} 0"))?;
        }
        output::read(&self.demo, &mut self.cache)
    }
    pub fn positions(&self) -> &[f32] {
        &self.cache.positions
    }
}
