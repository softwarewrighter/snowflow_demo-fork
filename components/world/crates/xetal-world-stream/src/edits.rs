//! Native dirty-region scheduling; X_eTaL owns brush shape, history and bounds.
use crate::{World, merge};
use xetal_world_model::{Demo, Progress};
use xetal_world_surface::Mesh;

impl World {
    pub fn sweep(&mut self, command: [f32; 5]) -> Result<(), String> {
        if command.iter().any(|v| !v.is_finite())
            || command[0] <= 0.0
            || command[0].fract() != 0.0
            || !(1.4..=15.0).contains(&command[4])
        {
            return Err("invalid Sweep terrain command".into());
        }
        self.pending_edit = Some(command);
        Ok(())
    }
    pub fn clear_edits(&mut self) {
        self.pending_edit = None;
        self.pending_reset = true;
    }
    pub(crate) fn apply_edits(&mut self) -> Result<Option<Mesh>, String> {
        if self.pending_reset {
            send(&mut self.demo, "key r")?;
            self.pending_reset = false;
        } else if let Some([id, x, z, yaw, reach]) = self.pending_edit.take() {
            for event in [
                format!("move {x} {z}"),
                format!("down {yaw} {reach}"),
                format!("up {id} 0"),
            ] {
                send(&mut self.demo, &event)?;
            }
        }
        let regions = regions(&self.demo)?;
        let keys: Vec<_> = self
            .chunks
            .keys()
            .copied()
            .filter(|&(x, z)| {
                regions.iter().any(|r| {
                    (x as f64 * 32.0) <= r[1]
                        && ((x as f64 + 1.0) * 32.0) >= r[0]
                        && (z as f64 * 32.0) <= r[3]
                        && ((z as f64 + 1.0) * 32.0) >= r[2]
                })
            })
            .collect();
        for key in &keys {
            send(&mut self.demo, &format!("choose {} {}", key.0, key.1))?;
            self.chunks.insert(*key, self.demo.mesh()?);
        }
        self.edit_revision += 1;
        if keys.is_empty() {
            return Ok(None);
        }
        merge::mesh(&self.chunks).map(Some)
    }
}
fn send(demo: &mut Demo, event: &str) -> Result<(), String> {
    demo.event(event)?;
    for _ in 0..100 {
        if demo.step(8_000)? == Progress::Waiting {
            return Ok(());
        }
    }
    Err("terrain edit did not settle within its execution budget".into())
}
fn regions(demo: &Demo) -> Result<Vec<[f64; 4]>, String> {
    let (shape, values) = demo.array(1)?;
    if shape.len() != 2 || shape[1] != 4 || shape[0] > 8 {
        return Err("invalid dirty terrain regions".into());
    }
    Ok(values.as_chunks::<4>().0.to_vec())
}
