//! Native event transport and checked typed Sweep output; policies live in X_eTaL.
mod output;
mod transport;
pub const SOURCE: &str = include_str!("../../../demos/sweep.xtl");
pub use output::Update;
pub use transport::Sweep;
