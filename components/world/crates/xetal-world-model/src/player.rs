//! Native pose transport only: X_eTaL libraries own footfalls, wind and sound commands.
use crate::{Commands, Demo, Progress};

#[derive(Clone, Copy)]
pub struct Pose { pub point: [f32;2], pub yaw: f32, pub grounded: bool, pub impact: f32 }
pub struct Player { demo: Demo, last: Pose, elapsed: f32, pose_elapsed: f32 }

impl Player {
    pub fn new() -> Result<Self, String> {
        let mut demo = Demo::new()?; settle(&mut demo)?;
        Ok(Self { demo, last: Pose { point: [0.0,8.0], yaw: 0.0, grounded: false, impact: 0.0 }, elapsed: 0.0, pose_elapsed: 0.0 })
    }
    pub fn update(&mut self, pose: Pose, dt: f32) -> Result<Option<Commands>, String> {
        self.elapsed += dt.clamp(0.0,0.1); self.pose_elapsed += dt.clamp(0.0,0.1);
        let mut changed = false;
        if self.elapsed >= 0.1 { self.demo.event(&format!("tick {}", self.elapsed))?; settle(&mut self.demo)?; self.elapsed = 0.0; changed = true; }
        let contact = pose.grounded != self.last.grounded || pose.impact > 0.0;
        if (pose.point != self.last.point || contact) && (self.pose_elapsed >= 0.05 || contact) {
            self.demo.event(&format!("move {} {}",pose.point[0],pose.point[1]))?; settle(&mut self.demo)?;
            self.demo.event(&format!("{} {} {}", if pose.grounded { "down" } else { "up" }, pose.yaw, pose.impact))?; settle(&mut self.demo)?;
            self.last = pose; self.pose_elapsed = 0.0; changed = true;
        }
        if changed { return self.demo.commands().map(Some); }
        Ok(None)
    }
    pub fn reset(&mut self, pose: Pose) -> Result<(), String> {
        self.demo.event(&format!("move {} {}",pose.point[0],pose.point[1]))?; settle(&mut self.demo)?;
        self.demo.event("click 0 0")?; settle(&mut self.demo)?;
        self.last = pose; self.elapsed = 0.0; self.pose_elapsed = 0.0;
        Ok(())
    }
}
fn settle(demo: &mut Demo) -> Result<(), String> {
    for _ in 0..100 {
        match demo.step(8_000)? { Progress::Waiting => return Ok(()), Progress::Running => {}, Progress::Done => return Err("player app ended".into()) }
    }
    Err("player app did not reach event wait".into())
}
