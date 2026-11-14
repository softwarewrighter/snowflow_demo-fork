//! Dispatch separate pending spell commands; preserve both when cast together.
use crate::{
    World,
    edits::{regions, send},
};
impl World {
    pub fn bloom(&mut self, command: [f32; 3]) -> Result<(), String> {
        if command.iter().any(|v| !v.is_finite()) || command[0] <= 0.0 || command[0].fract() != 0.0
        {
            return Err("invalid Bloom terrain command".into());
        }
        self.pending_bloom = Some(command);
        Ok(())
    }
    pub(crate) fn send_pending(&mut self) -> Result<Vec<[f64; 4]>, String> {
        let mut dirty = Vec::new();
        if self.pending_reset {
            send(&mut self.demo, "key r")?;
            self.pending_reset = false;
            dirty.extend(regions(&self.demo)?);
        }
        if let Some([id, x, z, yaw, reach]) = self.pending_edit.take() {
            for event in [
                format!("move {x} {z}"),
                format!("down {yaw} {reach}"),
                format!("up {id} 0"),
            ] {
                send(&mut self.demo, &event)?;
            }
            dirty.extend(regions(&self.demo)?);
        }
        if let Some([id, x, z]) = self.pending_bloom.take() {
            send(&mut self.demo, &format!("move {x} {z}"))?;
            send(&mut self.demo, &format!("click {id} 0"))?;
            dirty.extend(regions(&self.demo)?);
        }
        Ok(dirty)
    }
}
