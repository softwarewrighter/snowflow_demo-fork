//! Bounded procedural object geometry rendered with a faceted ice material.
pub const SHADER: &str = include_str!("ice.wgsl");
pub const MAX_VERTICES: usize = 4 * 12 * 18 * 3;
#[cfg(target_arch = "wasm32")]
mod gpu;
#[cfg(target_arch = "wasm32")]
mod pipeline;
#[cfg(target_arch = "wasm32")]
pub use gpu::Objects;

pub fn validate(vertices: &[f32]) -> Result<(), String> {
    if vertices.len() > MAX_VERTICES * 3
        || !vertices.len().is_multiple_of(9)
        || vertices.iter().any(|v| !v.is_finite())
    {
        return Err("invalid or oversized object triangle payload".into());
    }
    Ok(())
}
