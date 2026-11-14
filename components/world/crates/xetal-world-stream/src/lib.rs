//! Bounded streaming of deterministic X_eTaL terrain chunks.
mod commands;
mod edits;
mod merge;
pub const SOURCE: &str = include_str!("../../../demos/terrain.xtl");
mod plan;
mod stream;
pub use plan::{Key, center, ring};
pub use stream::World;
