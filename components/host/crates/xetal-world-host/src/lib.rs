//! Application mesh ownership and bounded resource lifetime, independent of a GPU.

mod error;
mod handle;
mod stats;
mod store;

pub use error::HostError;
pub use handle::MeshHandle;
pub use stats::{Limits, Stats};
pub use store::MeshStore;
