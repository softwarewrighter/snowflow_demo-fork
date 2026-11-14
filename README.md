# X_eTaL snowy world

[GitHub Pages preview](https://softwarewrighter.github.io/snowflow_demo-fork/)

An endless streamed snowfield built with [X_eTaL](https://github.com/softwarewrighter/X_eTaL) and Rust for the browser.
Walk across deterministic snowy hills and ice formations, leave a trail of
footprints, jump and hear snow crunches and landing thuds. The start screen
offers sound-enabled or muted entry. Terrain stays still while you rest.

Pure X_eTaL libraries generate world-space terrain, decide footfalls and sound
commands, and age prints under wind gusts. See the [library cross-reference](docs/libraries.md).
The displayed source imports the actual modules, and its rendered library calls
link to generated API/source documentation. A bounded
5 by 5 window follows the player and generates the next horizon ahead of
movement. Rust supplies browser/GPU integration, typed data transfer, navigation
physics, audio execution and footprint rendering. Application algorithms remain
candidates for reusable X_eTaL libraries as the host interface evolves.
The build uses generated loader glue with no handwritten application JavaScript
or WebAssembly.

The longer-term world includes digging, dirt, gems, water and torches. Snowflow
provides the visual reference; the X_eTaL endless-world demos inform gameplay.
The repository includes checked mesh payloads, repeatable tooling and an intact
archived reference. See [streaming, tracks and sound](docs/streaming-snow.md) for
current limits, ownership and evidence.

Run `python3 scripts/serve-demo.py --no-build` to serve the existing build at
[localhost:8475](http://127.0.0.1:8475/snowflow_demo-fork/).
For a browser on another machine, use the printed Network demo URL; the launcher
binds to all interfaces by default.
See [local demo](docs/local-demo.md) for a clean build and browser validation.

## Repository

- [Components](components/README.md): independent Rust workspaces and X_eTaL libraries.
- [Architecture](docs/architecture.md): ownership and execution boundaries.
- [Mesh contract](docs/mesh-contract.md): arrays, precision and resource lifetime.
- [Development](docs/development.md): checks, source pins and reproducible evidence.
- [Design](docs/design.md): application decisions and upstream language asks.
- [Plan](docs/plan.md): milestones and agentrail execution.
- [Pages publishing](docs/pages.md): hosted HTTPS preview and update process.
- [Original README](orig-README.md): the upstream Snowflow description.
- [Archived reference](archive/README.md): original sources, shaders and license.

## Checks

Run `bash scripts/gate.sh` from the repository root. The gate verifies archive
integrity, owned documentation, component formatting, lint and tests. Run
`python3 scripts/inspect-xetal.py` separately to check locally available research
snapshots against the source manifest. The gate does not require those research
snapshots, but it does build the pinned X_eTaL dependencies under `work/deps/`.

## License

New project files use [MIT](LICENSE); see [COPYRIGHT](COPYRIGHT).
Archived Snowflow files retain their [original MIT notice](archive/snowflow-original/LICENSE).

Copyright (c) 2026 Michael A Wright.

The preview also includes procedural material detail, a cached sun shadow map
and Space-to-jump player physics. Textures and Shadows can be disabled separately.
[Effects and performance checks](docs/lightweight-effects.md) describe their cost
bounds and pending hardware-browser measurements.

## Ice spell

Press **4** or use **4: Crystallise** to grow twelve ice crystals ahead of you.
The latest four formations remain; Reset clears them. The spell source is
shown as typed and as rendered below the demo, and its library functions link
to the API/source cross-reference. Geometry and growth run in X_eTaL; Rust
provides input transport and a bounded WebGPU ice renderer. Crystals are visual
objects and do not yet block movement.

## Sweep spell

Press **1** or click **1: Sweep** to launch a slush crescent that carves a channel
and raises snow banks. Walk into the result: collision uses the edited mesh.
The latest eight cuts remain, including after chunks reload; Reset clears them.
The live source panel and [Sweep documentation](docs/sweep.md) identify the
X_eTaL libraries, native backends, repeatable tests and current visual limits.

## Bloom spell

Aim the crosshair at the snow and press **3**, or click **3: Bloom**, to erupt a
column, carve a crater with a raised rim, and release falling snowflake particles.
The latest four craters remain. Reset clears them. Timing, crater shape and
particle motion live in X_eTaL libraries; a reusable Rust/WGPU particle renderer
provides the native drawing capability. See [Bloom](docs/bloom.md).

## Ribbon and snow mechanics

Hold **2** to steer Ribbon, then release to throw it along your aim. Click the
Ribbon button to start a latched hold and click again to throw. The stream leaves
shallow scores when its tip contacts loaded snow. See [Ribbon](docs/ribbon.md).
The [snow mechanics roadmap](docs/snow-mechanics.md) describes planned geometric
boot prints, compaction, deep-powder movement and persistent weathered snow tiles.
