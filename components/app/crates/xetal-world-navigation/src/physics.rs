//! Pure vertical motion; the host schedules fixed 120 Hz updates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    pub height: f32,
    pub velocity: f32,
    pub grounded: bool,
}

impl Motion {
    pub fn contact(self, floor: f32, jumping: bool) -> Self {
        if self.height < floor || (self.grounded && !jumping && self.height - floor <= 0.4) {
            return Self {
                height: floor,
                velocity: 0.0,
                grounded: true,
            };
        }
        self
    }

    pub fn step(self, seconds: f32, floor: f32, jump: bool) -> Self {
        let dt = seconds.clamp(0.0, 0.1);
        let velocity = if jump && self.grounded {
            5.2
        } else {
            self.velocity
        };
        let height = self.height + velocity * dt - 0.5 * 9.81 * dt * dt;
        if height <= floor {
            return Self {
                height: floor,
                velocity: 0.0,
                grounded: true,
            };
        }
        Self {
            height,
            velocity: velocity - 9.81 * dt,
            grounded: false,
        }
    }
}
