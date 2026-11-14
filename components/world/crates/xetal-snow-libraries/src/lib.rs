//! The actual embedded X_eTaL modules imported by the application.
use xetal_macro::{Found, Libraries, StoreLibraries};
pub struct Library { pub name: &'static str, pub alias: &'static str, pub anchor: &'static str, pub path: &'static str, pub source: &'static str }
pub const LIBRARIES: &[Library] = &[
    Library { name: "SnowTerrain", alias: "snow", anchor: "snow-terrain", path: "components/world/libs/SnowTerrain.xtl", source: include_str!("../../../libs/SnowTerrain.xtl") },
    Library { name: "Footsteps", alias: "feet", anchor: "footsteps", path: "components/world/libs/Footsteps.xtl", source: include_str!("../../../libs/Footsteps.xtl") },
    Library { name: "SnowAudio", alias: "sfx", anchor: "snow-audio", path: "components/world/libs/SnowAudio.xtl", source: include_str!("../../../libs/SnowAudio.xtl") },
    Library { name: "SnowHost", alias: "host", anchor: "snow-host", path: "components/world/libs/SnowHost.xtl", source: include_str!("../../../libs/SnowHost.xtl") },
    Library { name: "SnowWind", alias: "wind", anchor: "snow-wind", path: "components/world/libs/SnowWind.xtl", source: include_str!("../../../libs/SnowWind.xtl") },
];
pub struct Embedded;
impl Libraries for Embedded {
    fn find(&self, spec: &str, from: &str) -> Option<Found> {
        let name = spec.rsplit('/').next().unwrap_or(spec).trim_end_matches(".xtl");
        LIBRARIES.iter().find(|lib| lib.name == name).map(|lib| Found { key: lib.path.into(), name: lib.path.into(), text: lib.source.into() }).or_else(|| StoreLibraries.find(spec, from))
    }
}
