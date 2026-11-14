use crate::{Pose, Ribbon, io, output::Cache};
impl Ribbon {
    pub fn press(&mut self, pose: Pose) -> Result<(), String> {
        self.pose(pose)?;
        io::send(&mut self.demo, "click 1 0")
    }
    pub fn release(&mut self, pose: Pose) -> Result<(), String> {
        self.pose(pose)?;
        io::send(&mut self.demo, "click 0 0")
    }
    pub fn clear(&mut self) -> Result<(), String> {
        io::send(&mut self.demo, "key r")?;
        self.cache = Cache::default();
        self.last = None;
        self.elapsed = 0.0;
        self.cleared = true;
        Ok(())
    }
    pub(crate) fn pose(&mut self, pose: Pose) -> Result<(), String> {
        pose.validate()?;
        if self.last == Some(pose) {
            return Ok(());
        }
        for event in [
            format!("move {} {}", pose.origin[0], pose.origin[2]),
            format!("down {} 0", pose.origin[1]),
            format!("up {} {}", pose.aim[0], pose.aim[1]),
        ] {
            io::send(&mut self.demo, &event)?;
        }
        self.last = Some(pose);
        Ok(())
    }
}
