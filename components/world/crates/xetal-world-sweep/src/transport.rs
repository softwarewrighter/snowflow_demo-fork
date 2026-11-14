//! Input and typed mesh transport; Sweep.xtl owns every spell rule.
use crate::{SOURCE, Update};
use xetal_world_model::{Demo, Progress};

pub struct Sweep {
    demo: Demo,
    positions: Vec<f32>,
    elapsed: f32,
    cleared: bool,
    brush: [f32; 5],
}
impl Sweep {
    pub fn new() -> Result<Self, String> {
        let mut demo = Demo::from_source(SOURCE)?;
        settle(&mut demo)?;
        Ok(Self {
            demo,
            positions: Vec::new(),
            elapsed: 0.0,
            cleared: false,
            brush: [0.0; 5],
        })
    }
    pub fn cast(&mut self, point: [f32; 2], yaw: f32) -> Result<(), String> {
        if !point.iter().chain([&yaw]).all(|v| v.is_finite()) {
            return Err("nonfinite spell pose".into());
        }
        for event in [
            format!("move {} {}", point[0], point[1]),
            format!("down {yaw} 0"),
            "click 0 0".into(),
        ] {
            self.demo.event(&event)?;
            settle(&mut self.demo)?;
        }
        Ok(())
    }
    pub fn tick(&mut self, dt: f32) -> Result<Update, String> {
        if !dt.is_finite() || dt < 0.0 {
            return Err("invalid sweep tick".into());
        }
        if self.cleared {
            self.cleared = false;
            return Ok(Update {
                mesh: Some(self.demo.mesh()?),
                brush: None,
            });
        }
        self.elapsed += dt.min(0.1);
        if self.elapsed < 0.1 {
            return Ok(Update::default());
        }
        self.demo.event(&format!("tick {}", self.elapsed))?;
        settle(&mut self.demo)?;
        self.elapsed = 0.0;
        crate::output::read(&self.demo, &mut self.positions, &mut self.brush)
    }
    pub fn clear(&mut self) -> Result<(), String> {
        self.demo.event("key r")?;
        settle(&mut self.demo)?;
        self.positions.clear();
        self.brush = [0.0; 5];
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
