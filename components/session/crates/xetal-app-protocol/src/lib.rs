//! Validated, spell-independent capabilities published by an X_eTaL app.
mod frame;
mod metadata;
mod resources;
mod value;
pub use frame::{Frame, decode};
pub use metadata::{Button, Source};
pub use resources::{Cache, Resource};
