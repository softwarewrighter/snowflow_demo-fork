//! Browser WebGPU renderer for validated application mesh payloads.
pub use xetal_world_surface::Mesh;
#[cfg(target_arch = "wasm32")]
mod device;
#[cfg(target_arch = "wasm32")]
mod frame;
#[cfg(target_arch = "wasm32")]
mod pipeline;
#[cfg(target_arch = "wasm32")]
mod resources;
#[cfg(target_arch = "wasm32")]
pub use device::Renderer;
