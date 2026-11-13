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
| app | Browser composition, input wiring and inspection UI |
| evidence | Replays, captures and performance evidence |

The initial Rust facades are intentionally empty and have no dependencies.
X_eTaL library space is under `world/libs/`. Reusable ecosystem features must
land in their owning sibling repository, then be consumed through pinned commits.
