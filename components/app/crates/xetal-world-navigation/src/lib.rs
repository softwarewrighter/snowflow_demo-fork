//! Finite terrain navigation for the browser host; coordinates are meters.
mod collision;
mod physics;
pub use physics::Motion;

pub struct Walker {
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    motion: Motion,
    accumulated: f32,
    jump_requested: bool,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            position: [0.0, 1.7, 8.0],
            yaw: 0.0,
            pitch: -0.12,
            motion: Motion {
                height: 1.7,
                velocity: 0.0,
                grounded: false,
            },
            accumulated: 0.0,
            jump_requested: false,
        }
    }
}

impl Walker {
    pub fn advance(&mut self, seconds: f32, forward: f32, sideways: f32) {
        let distance = 3.5 * seconds.clamp(0.0, 0.1) / forward.hypot(sideways).max(1.0);
        let (s, c) = self.yaw.sin_cos();
        self.position[0] =
            (self.position[0] + distance * (sideways * c + forward * s)).clamp(-15.0, 15.0);
        self.position[2] =
            (self.position[2] + distance * (sideways * s - forward * c)).clamp(-15.0, 15.0);
    }

    pub fn ground(&mut self, positions: &[f32]) {
        self.accumulated = 0.0;
        self.jump_requested = false;
        if let Some(floor) = collision::floor(positions, self.position) {
            self.position[1] = floor;
            self.motion = Motion {
                height: floor,
                velocity: 0.0,
                grounded: true,
            };
        }
    }

    pub fn simulate(&mut self, seconds: f32, jump: bool, positions: &[f32]) {
        let Some(floor) = collision::floor(positions, self.position) else {
            return;
        };
        self.accumulated += seconds.clamp(0.0, 0.1);
        self.jump_requested |= jump;
        if self.motion.height < floor {
            self.motion = Motion {
                height: floor,
                velocity: 0.0,
                grounded: true,
            };
        }
        while self.accumulated >= 1.0 / 120.0 {
            self.motion = self.motion.step(1.0 / 120.0, floor, self.jump_requested);
            self.jump_requested = false;
            self.accumulated -= 1.0 / 120.0;
        }
        self.position[1] = self.motion.height;
    }
}
