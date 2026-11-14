//! Cached directional depth map, regenerated only when geometry changes.
#[cfg(target_arch = "wasm32")]
mod map;
#[cfg(target_arch = "wasm32")]
mod pipeline;
#[cfg(target_arch = "wasm32")]
mod resources;
#[cfg(target_arch = "wasm32")]
pub use map::Shadow;
