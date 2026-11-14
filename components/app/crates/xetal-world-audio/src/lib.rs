//! Deterministic sound synthesis with gesture-created WebAudio at the boundary.
mod synth;
pub use synth::{Kind, samples};
#[cfg(target_arch = "wasm32")]
mod bank;
#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg(target_arch = "wasm32")]
pub use browser::Sound;
