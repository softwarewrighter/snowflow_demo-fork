# X_eTaL snowy world

A procedural, editable snow world built with X_eTaL and Rust for the browser.
The world combines endless terrain, smooth snow and ice boulders, flowing water,
torches, exposed dirt, digging and buried gems. Snowflow provides the visual
reference; the X_eTaL endless-world demos provide the gameplay foundation.

Application algorithms belong in X_eTaL libraries. Rust supplies browser and GPU
integration, workers, typed data transfer and reusable execution mechanisms.
The browser build uses generated loader glue, with no handwritten application
JavaScript or WebAssembly.

The repository currently contains component scaffolding, design documentation,
source inspection tooling and an intact archived reference. There is no new
playable browser application yet.

## Repository

- [Components](components/README.md): independent Rust workspaces and X_eTaL libraries.
- [Architecture](docs/architecture.md): ownership and execution boundaries.
- [Development](docs/development.md): checks, source pins and reproducible evidence.
- [Design](docs/design.md): application decisions and upstream language asks.
- [Plan](docs/plan.md): milestones and agentrail execution.
- [Original README](orig-README.md): the upstream Snowflow description.
- [Archived reference](archive/README.md): original sources, shaders and license.

## Checks

Run `bash scripts/gate.sh` from the repository root. The gate verifies archive
integrity, owned documentation, component formatting, lint and tests. Run
`python3 scripts/inspect-xetal.py` separately to check locally available research
snapshots against the source manifest. The gate does not require those ignored
snapshots or compile sibling repositories.

## License

New project files use [MIT](LICENSE); see [COPYRIGHT](COPYRIGHT).
Archived Snowflow files retain their [original MIT notice](archive/snowflow-original/LICENSE).

Copyright (c) 2026 Michael A Wright.
