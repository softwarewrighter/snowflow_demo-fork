# Bloom

Enter the demo, aim the center crosshair at the snow, then press **3** or click
**3: Bloom**. A column of slush rises, withdraws into a crater, and leaves falling
snowflake particles. The crater has a raised rim and changes player collision.
Press 1 for Sweep or 4 for Crystallise. Pause freezes effects; Reset clears them.

## Actual libraries and backends

[Bloom.xtl](../components/world/libs/Bloom.xtl) defines casting, the column
spine/envelope, a single crater command and a bounded 512-particle fallout field.
The column collapses by 1.75 seconds. Particles are born over the eruption and
have four-second lifetimes; all fallout ends by 5.75 simulated seconds.
Recasting replaces the active column and fallout. The latest four craters remain.

[SnowParticles.xtl](../components/world/libs/SnowParticles.xtl) supplies reusable
pure seeded variation, ballistic motion, fade and six-column render rows:
`[x y z size opacity spin]`. [SnowWind](../components/world/libs/SnowWind.xtl)
supplies analytic gust strength. [SweptSurface](../components/world/libs/SweptSurface.xtl)
now also sweeps radial tube sections along a spine.

[BloomBrush](../components/world/libs/BloomBrush.xtl) uses the shared signed
TerrainBrush radial profile. Its immutable history, eviction and dirty bounds
feed the same streamed terrain program used by Sweep. Both spell commands can
be pending together. Reset collects all dirty bounds and restores base terrain.

The [composition program](../components/world/demos/bloom.xtl) is displayed live
as typed and as rendered, with clickable actual library references. Rust's
`xetal-world-bloom` transports facts and validates typed outputs. Rust's
`xetal-world-particles` renders up to 1024 instanced camera-facing procedural
snowflakes with a fixed 24,576-byte GPU buffer and one draw call. Bloom uses at
most 512 of those instances and 12,288 bytes per particle upload.

## Targeting and shared resources

The native physics ray query finds the nearest loaded terrain triangle under
the crosshair. Range and fallback distance come from the Bloom library: 24 and
6 meters. A missed ray falls back to loaded ground ahead. Crystal objects are
visual and are not ray targets. The actual hit height feeds the spell library.

Bloom has at most 480 triangles and shares the slush buffer/material with Sweep.
Both together fit the existing 864-triangle capacity. Four Bloom craters and
eight Sweep cuts are retained; evicting a crater restores its region. Rendered
terrain and collision share the committed mesh, and footprints reproject after
edits. Terrain still uses a two-meter grid and synchronous dirty-chunk rebuilds.

## Reproduction and limitations

Build using `python3 scripts/build-demo.py`; publish the buildable preview before
running `bash scripts/gate.sh`, per the project preview policy. Native tests cover
burst count, deterministic descending particles, lifetime, caps, crater/rim signs,
mixed spells, eviction, reset, targeting, pause and shader/upload validation.
Actual device rendering, audible quality and FPS remain separate browser checks.

Particles are alpha blended and depth tested without individual shadows or
sorting. Their analytic ground fade uses the target height; surrounding slopes
occlude particles through the depth buffer. The material does not reproduce the
original refraction, dynamic spell lights or full water/foam quality. Bloom has
no dedicated sound. Ambient snowfall is still queued; this renderer and the
SnowParticles library are the reusable foundation for snowfall and Vortex.
