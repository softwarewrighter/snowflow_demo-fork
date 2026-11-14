//! Input and typed mesh transport; Bloom.xtl owns every spell rule.
use crate::{SOURCE, Update};
use xetal_world_model::{Demo, Progress};

pub struct Bloom {
    demo: Demo,
    pub(crate) positions: Vec<f32>,
    elapsed: f32,
    cleared: bool,
    pub(crate) brush: [f32; 3],
    pub(crate) particles: Vec<f32>,
    pub(crate) targeting: [f32; 2],
}
impl Bloom {
    pub fn new() -> Result<Self, String> {
        let mut demo = Demo::from_source(SOURCE)?;
        settle(&mut demo)?;
        let targeting = crate::output::targeting(&demo)?;
        Ok(Self {
            demo,
            positions: Vec::new(),
            elapsed: 0.0,
            cleared: false,
            brush: [0.0; 3],
            particles: Vec::new(),
            targeting,
        })
    }
    pub fn cast(&mut self, target: [f32; 3]) -> Result<(), String> {
        if target.iter().any(|v| !v.is_finite()) {
            return Err("nonfinite Bloom target".into());
        }
        for event in [
            format!("move {} {}", target[0], target[2]),
            format!("down {} 0", target[1]),
            "click 0 0".into(),
        ] {
            self.demo.event(&event)?;
            settle(&mut self.demo)?;
        }
        Ok(())
    }
    pub fn tick(&mut self, dt: f32) -> Result<Update, String> {
        if !dt.is_finite() || dt < 0.0 {
            return Err("invalid Bloom tick".into());
        }
        if self.cleared {
            self.cleared = false;
            return Ok(Update {
                mesh: Some(self.demo.mesh()?),
                crater: None,
                particles: Some(Vec::new()),
            });
        }
        self.elapsed += dt.min(0.1);
        if self.elapsed < 0.05 {
            return Ok(Update::default());
        }
        self.demo.event(&format!("tick {}", self.elapsed))?;
        settle(&mut self.demo)?;
        self.elapsed = 0.0;
        crate::output::read(
            &self.demo,
            &mut self.positions,
            &mut self.particles,
            &mut self.brush,
        )
    }
    pub fn clear(&mut self) -> Result<(), String> {
        self.demo.event("key r")?;
        settle(&mut self.demo)?;
        self.positions.clear();
        self.brush = [0.0; 3];
        self.particles.clear();
        self.cleared = true;
        self.elapsed = 0.0;
        Ok(())
    }
    pub fn positions(&self) -> &[f32] {
        &self.positions
    }
}
fn settle(demo: &mut Demo) -> Result<(), String> {
    for _ in 0..100 {
        match demo.step(8_000)? {
            Progress::Waiting => return Ok(()),
            Progress::Running => {}
            Progress::Done => return Err("spell app ended".into()),
        }
    }
    Err("spell app did not reach event wait".into())
}
