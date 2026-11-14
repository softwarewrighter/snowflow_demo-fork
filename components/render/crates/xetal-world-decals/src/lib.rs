//! Bounded procedural snow-print decals using the terrain camera ABI.
pub const SHADER: &str = include_str!("footprints.wgsl");
#[cfg(target_arch = "wasm32")]
mod gpu;
#[cfg(target_arch = "wasm32")]
mod pipeline;
#[cfg(target_arch = "wasm32")]
pub use gpu::Decals;
