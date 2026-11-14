//! Finite terrain navigation for the browser host; coordinates are meters.
pub struct Walker {
    pub position: [f32; 3],
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            position: [0.0, 1.7, 8.0],
            yaw: 0.0,
            pitch: -0.12,
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
        for triangle in positions.as_chunks::<9>().0.iter() {
            if let Some(height) = height(triangle, self.position[0], self.position[2]) {
                self.position[1] = height + 1.7;
                return;
            }
        }
    }
}

fn height(p: &[f32], x: f32, z: f32) -> Option<f32> {
    let (ax, az, bx, bz) = (p[3] - p[0], p[5] - p[2], p[6] - p[0], p[8] - p[2]);
    let det = ax * bz - az * bx;
    if det.abs() < 1e-8 {
        return None;
    }
    let u = ((x - p[0]) * bz - (z - p[2]) * bx) / det;
    let v = (ax * (z - p[2]) - az * (x - p[0])) / det;
    (u >= -1e-5 && v >= -1e-5 && u + v <= 1.00001)
        .then(|| p[1] + u * (p[4] - p[1]) + v * (p[7] - p[1]))
}
