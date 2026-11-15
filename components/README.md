# Components

Each component is an independent Cargo workspace with small crates under
`crates/`, matching the X_eTaL repository layout. There is no root workspace.
Build from the repository root with `--manifest-path components/NAME/Cargo.toml`
or from inside a component; `.cargo/config.toml` selects the shared `target/`.

| Component | Responsibility |
| --- | --- |
| host | Typed host boundary, events, workers and resource lifecycle |
| world | Density, materials, chunks, editing, water and gameplay contracts |
| surface | Smooth meshing and surface contracts |
| render | GPU buffers, textures, pipelines and rendering integration |
| session | One application evaluator, validated publication protocol and bounded chunk residency |
| app | Browser composition, input wiring and inspection UI |
| evidence | Replays, captures and performance evidence |

Host and surface implement the application mesh contract. World runs the X_eTaL
surface program; render supplies wgpu; app composes the browser experience.
Evidence remains a placeholder for shared replay/reporting code.
X_eTaL library space is under `world/libs/`. Reusable ecosystem features must
land in their owning sibling repository, then be consumed through pinned commits.
