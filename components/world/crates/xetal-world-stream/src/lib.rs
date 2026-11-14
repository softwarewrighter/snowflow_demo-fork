//! Bounded streaming of deterministic X_eTaL terrain chunks.
mod merge;
mod plan;
mod stream;
pub use plan::{Key, center, ring};
pub use stream::World;
