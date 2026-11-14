//! Reusable bounded instanced snow particles, driven by X_eTaL numeric rows.
pub const SHADER: &str = include_str!("particles.wgsl");
pub const CAPACITY: usize = 1024;
#[cfg(target_arch = "wasm32")]
mod gpu;
#[cfg(target_arch = "wasm32")]
mod pipeline;
#[cfg(target_arch = "wasm32")]
pub use gpu::Particles;
pub fn validate(rows: &[f32]) -> Result<(), String> {
    if rows.len() > CAPACITY * 6
        || !rows.len().is_multiple_of(6)
        || rows.iter().any(|v| !v.is_finite())
    {
        return Err("invalid particle buffer shape or size".into());
    }
    if rows
        .as_chunks::<6>()
        .0
        .iter()
        .any(|p| p[3] <= 0.0 || p[3] > 2.0 || !(0.0..=1.0).contains(&p[4]))
    {
        return Err("invalid particle size or opacity".into());
    }
    Ok(())
}
