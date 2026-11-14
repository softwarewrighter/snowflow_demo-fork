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
    highlighted_text(SOURCE)
}

mod frame;
mod player;
pub use frame::Commands;
pub use player::{Player, Pose};

/// Render actual library calls as links to their generated cross-reference.
pub fn highlighted_text(source: &str) -> String {
    xetal_view::view(source).into_iter().map(|segment| {
        let raw = &source[segment.raw.start..segment.raw.end];
        let library = xetal_snow_libraries::LIBRARIES.iter().find(|lib| raw.starts_with(&format!("{}:", lib.alias)));
        let html = xetal_view::html(&[segment]);
        match library { Some(lib) => format!("<a href=\"library-reference.html#{}\" title=\"{} API and source\" target=\"_blank\" rel=\"noopener noreferrer\">{html}</a>", lib.anchor, lib.name), None => html }
    }).collect()
}
