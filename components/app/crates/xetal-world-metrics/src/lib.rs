//! Pure rolling animation-frame measurements, independent of GPU timing.
#[derive(Clone, Copy, Default)]
pub struct Meter {
    start: Option<f64>,
    last: f64,
    frames: u32,
    worst: f64,
}
pub struct Reading {
    pub fps: f64,
    pub mean_ms: f64,
    pub worst_ms: f64,
}
impl Meter {
    pub fn record(mut self, now: f64) -> (Self, Option<Reading>) {
        if !now.is_finite() {
            return (self, None);
        }
        let Some(start) = self.start else {
            self.start = Some(now);
            self.last = now;
            return (self, None);
        };
        let interval = now - self.last;
        if !(0.0..=5000.0).contains(&interval) {
            return (
                Self {
                    start: Some(now),
                    last: now,
                    ..Self::default()
                },
                None,
            );
        }
        self.frames += 1;
        self.worst = self.worst.max(interval);
        self.last = now;
        let elapsed = now - start;
        if elapsed < 500.0 {
            return (self, None);
        }
        let reading = Reading {
            fps: f64::from(self.frames) * 1000.0 / elapsed,
            mean_ms: elapsed / f64::from(self.frames),
            worst_ms: self.worst,
        };
        self.start = Some(now);
        self.frames = 0;
        self.worst = 0.0;
        (self, Some(reading))
    }
}
