# X_eTaL snowy world

[GitHub Pages preview](https://softwarewrighter.github.io/snowflow_demo-fork/)

A procedural, editable snow world built with X_eTaL and Rust for the browser.
The world combines endless terrain, smooth snow and ice boulders, flowing water,
torches, exposed dirt, digging and buried gems. Snowflow provides the visual
reference; the X_eTaL endless-world demos provide the gameplay foundation.

Application algorithms belong in X_eTaL libraries. Rust supplies browser and GPU
integration, workers, typed data transfer and reusable execution mechanisms.
The browser build uses generated loader glue, with no handwritten application
JavaScript or WebAssembly.

The first local application renders an X_eTaL-generated snow patch with pause,
ridge input, reset and an orbit camera. The endless world is not implemented yet.
The repository also contains validated mesh payloads, a bounded resource store,
reproducible build tooling and an intact archived reference.

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
