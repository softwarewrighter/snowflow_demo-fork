# Sweep

Enter the demo, face the desired direction with the arrow keys, then press 1 or
click **1: Sweep**. A curled slush crescent travels away from the player, opens
outward and collapses after 2.4 simulated seconds. It leaves a broad channel
with raised snow banks. Recasting restarts the active wave and starts a new cut.
Pause freezes the effect. Reset clears both spells and all terrain edits.

## Library ownership

- [Sweep](../components/world/libs/Sweep.xtl): casting, decaying trajectory,
  crescent spine, envelope and semantic brush command.
- [SweptSurface](../components/world/libs/SweptSurface.xtl): reusable curled
  section meshing along an N-row spine.
- [TerrainBrush](../components/world/libs/TerrainBrush.xtl): pure signed cut/bank
  profile and bounded total displacement.
- [SweepBrush](../components/world/libs/SweepBrush.xtl): curved cut field,
  immutable latest-eight history, replacement and dirty-region bounds.
- [SnowTerrain](../components/world/libs/SnowTerrain.xtl): base and edited chunk
  meshes from the same world-space coordinates.

The [Sweep program](../components/world/demos/sweep.xtl) publishes a typed mesh
and brush command. The [terrain program](../components/world/demos/terrain.xtl)
maintains immutable brush history and publishes mesh/dirty bounds. The live
Sweep source is shown as typed and as rendered, with actual library links.
Rust schedules dirty chunks and uploads validated geometry; it does not choose
the trajectory, brush shape, lifetime or retention count.

## Collision, borders and retention

Dirty resident chunks rebuild as one edit batch. Rendering and player collision
consume the same committed triangle payload. Footprint decals reproject onto
changed terrain. Footprints currently remain decals; they do not yet write
physical depressions through TerrainBrush. The shared brush library establishes
the signed edit foundation for subsequent spells and digging.

Retained edits replay when chunks stream back in. There are at most 25 resident
chunks, one pending coalesced brush command and eight retained cast descriptors.
The ninth cast removes the oldest cut and restores its resident region. This is
bounded preview retention, not an unlimited saved world. A pending chunk job can
delay applying the latest brush until the next update.

## Cost and parity limits

The wave has at most 384 triangles, one extra draw call and a fixed 31,104-byte
GPU vertex buffer (13,824 occupied bytes at full wave size). Edited terrain uses
the existing chunk resolution and cached sun shadows update after mesh edits.

Native release CPU Scene updates measured a median 16.132 ms for the first cast
and 38.894 ms with eight retained cuts (p95 40.131 ms for the eighth cast),
without footprints or GPU work. Full samples are in [the evidence](evidence/sweep.json). Rebuilding
edited terrain currently blocks the main thread and can cause frame spikes,
especially with multiple retained cuts. Worker execution and faster array
kernels remain necessary optimizations; these measurements are not browser FPS.

This first port uses broad channels appropriate to the two-meter terrain grid,
and a nearly opaque procedural slush shader. It does not reproduce the original
fine brush spacing, conserved snow mass, particle spray, dynamic spell lights,
refraction or full material fidelity. Crystal bases do not reproject when a
later Sweep edits the ground beneath them. Existing walking/landing sounds are
preserved; Sweep has no dedicated sound yet.

## Reproduction

Build the release preview with `python3 scripts/build-demo.py`. Run the normal
post-publication gate with `bash scripts/gate.sh`. Native tests cover channel and
bank signs, unaffected points, reset, exact chunk seams, retained-edit replay,
replacement/eviction, bounded wave lifetime and player contact on edited terrain.
The browser harness checks casting and edit counters and captures a screenshot;
actual device appearance and FPS remain separate acceptance evidence.
