use crate::geometry;
use std::collections::VecDeque;

pub struct Mark {
    pub position: [f32; 3],
    pub yaw: f32,
    pub vertices: [f32; 30],
}
pub struct Trail {
    pub marks: VecDeque<Mark>,
    last: [f32; 3],
    travel: f32,
    side: f32,
    pub steps: u64,
    pub revision: u64,
}

impl Trail {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            marks: VecDeque::new(),
            last: position,
            travel: 0.0,
            side: 1.0,
            steps: 0,
            revision: 0,
        }
    }

    pub fn update(
        &mut self,
        position: [f32; 3],
        yaw: f32,
        grounded: bool,
        landing: f32,
        terrain: &[f32],
    ) -> bool {
        let distance = (position[0] - self.last[0]).hypot(position[2] - self.last[2]);
        self.last = position;
        if !grounded || distance > 2.0 {
            self.travel = 0.0;
            return false;
        }
        if landing > 1.0 {
            self.push(position, yaw, -1.0, terrain);
            self.push(position, yaw, 1.0, terrain);
            self.travel = 0.0;
            return false;
        }
        self.travel += distance;
        if self.travel < 0.9 {
            return false;
        }
        self.travel -= 0.9;
        self.push(position, yaw, self.side, terrain);
        self.side = -self.side;
        self.steps += 1;
        true
    }

    fn push(&mut self, mut position: [f32; 3], yaw: f32, side: f32, terrain: &[f32]) {
        position[0] += yaw.cos() * 0.12 * side;
        position[2] += yaw.sin() * 0.12 * side;
        let Some(vertices) = geometry::quad(position, yaw, terrain) else {
            return;
        };
        self.marks.push_back(Mark {
            position,
            yaw,
            vertices,
        });
        if self.marks.len() > 256 {
            self.marks.pop_front();
        }
        self.revision += 1;
    }

    pub fn vertices(&self) -> Vec<f32> {
        self.marks.iter().flat_map(|mark| mark.vertices).collect()
    }
}
