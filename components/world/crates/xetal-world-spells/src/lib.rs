//! Input and typed mesh transport; Crystallise.xtl owns every spell rule.
use xetal_world_model::{Demo, Progress};
use xetal_world_surface::Mesh;
pub const SOURCE: &str = include_str!("../../../demos/crystallise.xtl");

pub struct Spells {
    demo: Demo,
    positions: Vec<f32>,
    elapsed: f32,
    cleared: bool,
}
impl Spells {
    pub fn new() -> Result<Self, String> {
        let mut demo = Demo::from_source(SOURCE)?;
        settle(&mut demo)?;
        Ok(Self {
            demo,
            positions: Vec::new(),
            elapsed: 0.0,
            cleared: false,
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
    pub fn tick(&mut self, dt: f32) -> Result<Option<Mesh>, String> {
        if !dt.is_finite() || dt < 0.0 {
            return Err("invalid spell tick".into());
        }
        if self.cleared {
            self.cleared = false;
            return self.demo.mesh().map(Some);
        }
        self.elapsed += dt.min(0.1);
        if self.elapsed < 0.1 {
            return Ok(None);
        }
        self.demo.event(&format!("tick {}", self.elapsed))?;
        settle(&mut self.demo)?;
        self.elapsed = 0.0;
        let mesh = self.demo.mesh()?;
        if mesh.indices().len() > 4 * 12 * 18 * 3 {
            return Err("spell mesh budget exceeded".into());
        }
        if mesh.positions() == self.positions {
            return Ok(None);
        }
        self.positions = mesh.positions().to_vec();
        Ok(Some(mesh))
    }
    pub fn clear(&mut self) -> Result<(), String> {
        self.demo.event("key r")?;
        settle(&mut self.demo)?;
        self.positions.clear();
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
