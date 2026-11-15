//! The actual embedded X_eTaL modules imported by the application.
use xetal_macro::{Found, Libraries, StoreLibraries};
pub struct Library {
    pub name: &'static str,
    pub alias: &'static str,
    pub anchor: &'static str,
    pub path: &'static str,
    pub source: &'static str,
}
pub const LIBRARIES: &[Library] = &[
    Library {
        name: "SnowLayer",
        alias: "layer",
        anchor: "snow-layer",
        path: "components/world/libs/SnowLayer.xtl",
        source: include_str!("../../../libs/SnowLayer.xtl"),
    },
    Library {
        name: "Vortex",
        alias: "vortex",
        anchor: "vortex",
        path: "components/world/libs/Vortex.xtl",
        source: include_str!("../../../libs/Vortex.xtl"),
    },
    Library {
        name: "VortexBrush",
        alias: "lift",
        anchor: "vortex-brush",
        path: "components/world/libs/VortexBrush.xtl",
        source: include_str!("../../../libs/VortexBrush.xtl"),
    },
    Library {
        name: "Controls",
        alias: "controls",
        anchor: "controls",
        path: "components/world/libs/Controls.xtl",
        source: include_str!("../../../libs/Controls.xtl"),
    },
    Library {
        name: "Player",
        alias: "player",
        anchor: "player",
        path: "components/world/libs/Player.xtl",
        source: include_str!("../../../libs/Player.xtl"),
    },
    Library {
        name: "World",
        alias: "world",
        anchor: "world",
        path: "components/world/libs/World.xtl",
        source: include_str!("../../../libs/World.xtl"),
    },
    Library {
        name: "Spells",
        alias: "spells",
        anchor: "spells",
        path: "components/world/libs/Spells.xtl",
        source: include_str!("../../../libs/Spells.xtl"),
    },
    Library {
        name: "AppFrame",
        alias: "app",
        anchor: "appframe",
        path: "components/world/libs/AppFrame.xtl",
        source: include_str!("../../../libs/AppFrame.xtl"),
    },
    Library {
        name: "RibbonBrush",
        alias: "score",
        anchor: "ribbon-brush",
        path: "components/world/libs/RibbonBrush.xtl",
        source: include_str!("../../../libs/RibbonBrush.xtl"),
    },
    Library {
        name: "Ribbon",
        alias: "ribbon",
        anchor: "ribbon",
        path: "components/world/libs/Ribbon.xtl",
        source: include_str!("../../../libs/Ribbon.xtl"),
    },
    Library {
        name: "RibbonSurface",
        alias: "stream",
        anchor: "ribbon-surface",
        path: "components/world/libs/RibbonSurface.xtl",
        source: include_str!("../../../libs/RibbonSurface.xtl"),
    },
    Library {
        name: "Bloom",
        alias: "bloom",
        anchor: "bloom",
        path: "components/world/libs/Bloom.xtl",
        source: include_str!("../../../libs/Bloom.xtl"),
    },
    Library {
        name: "BloomBrush",
        alias: "crater",
        anchor: "bloom-brush",
        path: "components/world/libs/BloomBrush.xtl",
        source: include_str!("../../../libs/BloomBrush.xtl"),
    },
    Library {
        name: "SnowParticles",
        alias: "powder",
        anchor: "snow-particles",
        path: "components/world/libs/SnowParticles.xtl",
        source: include_str!("../../../libs/SnowParticles.xtl"),
    },
    Library {
        name: "TerrainBrush",
        alias: "brush",
        anchor: "terrain-brush",
        path: "components/world/libs/TerrainBrush.xtl",
        source: include_str!("../../../libs/TerrainBrush.xtl"),
    },
    Library {
        name: "SweepBrush",
        alias: "carve",
        anchor: "sweep-brush",
        path: "components/world/libs/SweepBrush.xtl",
        source: include_str!("../../../libs/SweepBrush.xtl"),
    },
    Library {
        name: "SweptSurface",
        alias: "tube",
        anchor: "swept-surface",
        path: "components/world/libs/SweptSurface.xtl",
        source: include_str!("../../../libs/SweptSurface.xtl"),
    },
    Library {
        name: "Sweep",
        alias: "sweep",
        anchor: "sweep",
        path: "components/world/libs/Sweep.xtl",
        source: include_str!("../../../libs/Sweep.xtl"),
    },
    Library {
        name: "Crystallise",
        alias: "ice",
        anchor: "crystallise",
        path: "components/world/libs/Crystallise.xtl",
        source: include_str!("../../../libs/Crystallise.xtl"),
    },
    Library {
        name: "SnowTerrain",
        alias: "snow",
        anchor: "snow-terrain",
        path: "components/world/libs/SnowTerrain.xtl",
        source: include_str!("../../../libs/SnowTerrain.xtl"),
    },
    Library {
        name: "Footsteps",
        alias: "feet",
        anchor: "footsteps",
        path: "components/world/libs/Footsteps.xtl",
        source: include_str!("../../../libs/Footsteps.xtl"),
    },
    Library {
        name: "SnowAudio",
        alias: "sfx",
        anchor: "snow-audio",
        path: "components/world/libs/SnowAudio.xtl",
        source: include_str!("../../../libs/SnowAudio.xtl"),
    },
    Library {
        name: "SnowHost",
        alias: "host",
        anchor: "snow-host",
        path: "components/world/libs/SnowHost.xtl",
        source: include_str!("../../../libs/SnowHost.xtl"),
    },
    Library {
        name: "SnowWind",
        alias: "wind",
        anchor: "snow-wind",
        path: "components/world/libs/SnowWind.xtl",
        source: include_str!("../../../libs/SnowWind.xtl"),
    },
];
pub struct Embedded;
impl Libraries for Embedded {
    fn find(&self, spec: &str, from: &str) -> Option<Found> {
        let name = spec
            .rsplit('/')
            .next()
            .unwrap_or(spec)
            .trim_end_matches(".xtl");
        LIBRARIES
            .iter()
            .find(|lib| lib.name == name)
            .map(|lib| Found {
                key: lib.path.into(),
                name: lib.path.into(),
                text: lib.source.into(),
            })
            .or_else(|| StoreLibraries.find(spec, from))
    }
}
