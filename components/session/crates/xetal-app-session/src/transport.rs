use crate::Session;
use xetal_world_model::Progress;
impl Session {
    pub(crate) fn settle(&mut self) -> Result<(), String> {
        for _ in 0..4096 {
            match self.demo.step(8000)? {
                Progress::Waiting => return Ok(()),
                Progress::Done => return Err("application stopped unexpectedly".into()),
                Progress::Running => (),
            }
        }
        Err("application instruction budget exceeded".into())
    }
    pub(crate) fn send(&mut self, event: &str) -> Result<(), String> {
        self.demo.event(event)?;
        self.settle()
    }
    pub fn detail_tile(&mut self, key: (i32, i32), spacing: f32) -> Result<Vec<f32>, String> {
        if !spacing.is_finite()
            || !(0.05..=0.1).contains(&spacing)
            || ((2.0 / spacing) - (2.0 / spacing).round()).abs() > 0.0001
        {
            return Err("invalid detail spacing".into());
        }
        self.send(&format!("move {} {}", key.0, key.1))?;
        self.send(&format!("click 6 {spacing}"))?;
        let (shape, values) = self.demo.array(0)?;
        let width = (2.0 / spacing).round() as usize + 1;
        if shape != [width * width, 3] {
            return Err("invalid detail tile shape".into());
        }
        Ok(values.into_iter().map(|n| n as f32).collect())
    }
    pub fn acknowledge(&mut self, version: u64) -> Result<(), String> {
        self.send(&format!("click 4 {version}"))
    }
    pub fn blur(&mut self) -> Result<(), String> {
        self.send("click 5 0")
    }
    pub fn respond(&mut self, id: u32, point: Option<[f32; 3]>) -> Result<(), String> {
        let found = u8::from(point.is_some());
        let p = point.unwrap_or([0.0; 3]);
        if p.iter().any(|n| !n.is_finite()) {
            return Err("nonfinite query response".into());
        }
        self.send(&format!("move {} {}", p[0], p[2]))?;
        self.send(&format!("down {} {found}", p[1]))?;
        self.send(&format!("click 3 {id}"))
    }
}
