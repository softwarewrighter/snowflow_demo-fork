//! Streamed terrain navigation for the browser host; coordinates are meters.
mod collision;
mod physics;
pub use collision::floor;
pub use physics::Motion;

pub struct Walker {
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
    pub motion: Motion,
    pub landing_speed: f32,
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
            landing_speed: 0.0,
            accumulated: 0.0,
            jump_requested: false,
        }
    }
}

impl Walker {
    pub fn advance(&mut self, seconds: f32, forward: f32, sideways: f32) {
        let distance = 3.5 * seconds.clamp(0.0, 0.1) / forward.hypot(sideways).max(1.0);
        let (s, c) = self.yaw.sin_cos();
        self.position[0] += distance * (sideways * c + forward * s);
        self.position[2] += distance * (sideways * s - forward * c);
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
        self.landing_speed = 0.0;
        let Some(floor) = collision::floor(positions, self.position) else {
            return;
        };
        self.accumulated += seconds.clamp(0.0, 0.1);
        self.jump_requested |= jump;
        self.motion = self.motion.contact(floor, self.jump_requested);
        while self.accumulated >= 1.0 / 120.0 {
            let before = self.motion;
            self.motion = before.step(1.0 / 120.0, floor, self.jump_requested);
            if !before.grounded && self.motion.grounded && before.velocity < -1.0 {
                self.landing_speed = -before.velocity;
            }
            self.jump_requested = false;
            self.accumulated -= 1.0 / 120.0;
        }
        self.position[1] = self.motion.height;
    }
}
