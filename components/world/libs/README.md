# World libraries

The demo imports these actual project-local X_eTaL libraries:

- SnowTerrain: deterministic snow and ice terrain geometry.
- Footsteps: walking, landing and footprint history.
- SnowAudio: semantic sound commands, gain and pitch.
- SnowWind: gust strength and footprint filling.
- SnowHost: the existing queued Event transport facade.
- Crystallise: spiral placement, hexagonal geometry and crystal growth.
- Sweep: moving crescent trajectory, lifetime and semantic edit command.
- SweptSurface: curled sheet meshing along a spine.
- TerrainBrush: signed channel/bank profile and displacement bounds.
- SweepBrush: curved terrain cuts, latest-eight history and dirty regions.
- Bloom: targeted eruption, crater command and particle fallout.
- BloomBrush: radial crater/rim field, latest-four history and dirty bounds.
- SnowParticles: pure particle kinematics, variation, fade and render rows.

See [library cross-reference](../../../docs/libraries.md) for public APIs,
Rust backends and tests. Reusable modules can move to their owning X_eTaL
repository through explicit dependency pin updates.
