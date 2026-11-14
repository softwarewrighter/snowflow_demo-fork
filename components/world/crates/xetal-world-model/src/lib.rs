//! A persistent sliced X_eTaL snow-surface program and typed snapshot adapter.
mod capture;
mod runtime;
mod snapshot;

pub use runtime::{Demo, Progress};
pub const SOURCE: &str = include_str!("../../../demos/snowfield.xtl");
