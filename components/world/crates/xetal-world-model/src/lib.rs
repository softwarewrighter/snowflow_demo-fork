//! A persistent sliced X_eTaL snow-surface program and typed snapshot adapter.
mod capture;
mod runtime;
mod snapshot;

pub use runtime::{Demo, Progress};
pub const SOURCE: &str = include_str!("../../../demos/snowfield.xtl");

/// Decorate the shipped source using the pinned language renderer.
pub fn rendered_source() -> Result<String, String> {
    xetal_render::decorate(SOURCE).map_err(|error| error.to_string())
}

/// Escaped, highlighted HTML using the same semantic view as the web editor.
pub fn highlighted_source() -> String {
    xetal_view::html(&xetal_view::view(SOURCE))
}
