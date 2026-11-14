# World libraries

The demo imports these actual project-local X_eTaL libraries:

- SnowTerrain: deterministic snow and ice terrain geometry.
- Footsteps: walking, landing and footprint history.
- SnowAudio: semantic sound commands, gain and pitch.
- SnowWind: gust strength and footprint filling.
- SnowHost: the existing queued Event transport facade.
- Crystallise: spiral placement, hexagonal geometry and crystal growth.

See [library cross-reference](../../../docs/libraries.md) for public APIs,
Rust backends and tests. Reusable modules can move to their owning X_eTaL
repository through explicit dependency pin updates.
