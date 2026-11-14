//! Validated application mesh payloads and adapters for X_eTaL numeric arrays.

mod adapters;
mod error;
mod mesh;
mod validate;

pub use adapters::{from_scene, from_xetal};
pub use error::MeshError;
pub use mesh::Mesh;
