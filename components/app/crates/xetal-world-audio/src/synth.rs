#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Step,
    Land,
}

pub fn samples(kind: Kind) -> Vec<f32> {
    let duration = if kind == Kind::Step { 0.09 } else { 0.2 };
    let mut seed = 0x12345678_u32;
    let mut low = 0.0;
    (0..(44100.0 * duration) as usize)
        .map(|i| {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = (seed >> 8) as f32 / 8388607.5 - 1.0;
            low += 0.06 * (noise - low);
            let t = i as f32 / 44100.0;
            let envelope = (t / 0.004).min(1.0) * (1.0 - t / duration).powi(3);
            let value = if kind == Kind::Step {
                noise * (0.35 + 0.65 * ((i % 211) as f32 / 211.0).powi(5))
            } else {
                1.2 * low + 0.35 * (std::f32::consts::TAU * 65.0 * t).sin()
            };
            value * envelope
        })
        .collect()
}
