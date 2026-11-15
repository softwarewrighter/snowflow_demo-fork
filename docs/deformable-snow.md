# Deformable snow layer

Walking and landing create actual fine-grid depressions with raised edges. The
existing snow material shades those triangles. The old flat print draw is
replaced by this geometry; print history and sound still come from X_eTaL.
This is an initial RDR2-inspired interaction milestone, not a claim about
Rockstar's internal algorithms or full visual parity.

## Libraries and host capabilities

[SnowLayer.xtl](../components/world/libs/SnowLayer.xtl) owns the footprint shape,
pressure depth, edge profile, overlap limits and aging strength. Its public APIs
are c_onfig, s_tamps, b_ounds and t_ile. App.xtl imports it, and AppFrame publishes
its grid configuration and bounded stamp descriptors. The live Snow layer source
panel and generated library reference expose the actual implementation.

The default configuration is 10 cm spacing, 2-meter tiles, a 6-meter local radius
and at most 36 resident tiles. Change the first value in layer:c_onfig from 0.1
to 0.0625 for 6.25 cm spacing and rebuild. Supported spacing must divide two
meters exactly and lie between 5 and 10 cm. Untouched cells retain the coarse
terrain; fine tiles replace cells touched by nearby stamps. This is a sparse
local field, not a subdivision of the entire endless world.

Each boot produces a rounded sole depression up to 5.5 cm deep and a raised
powder rim. Overlapping impressions clamp at 7.5 cm of depression and 2.5 cm of
raised displacement. This is a bounded geometric pressure rule. It does not yet
track snow mass, density, compaction, wetness or deep-powder resistance.

The [Rust tile backend](../components/snow/crates/xetal-snow-layer/src/layer.rs)
provides residency, bounded buffers, static grid topology, projection onto the
current coarse triangle surface and spatial sampling. It builds at most one
X_eTaL tile per update. Completed batches replace the visible and collision
snapshot together; its version appears in data-snow-version. New inputs cancel
obsolete batches. Rapid changes can defer a commit until a batch completes.
Execution remains on the main thread; worker execution is future work.

Fine collision uses the same triangle diagonal and committed heights as the
indexed render mesh. Native walking receives that sampled floor. Spell raycasts
and terrain shadow maps also use the composed geometry. A changed coarse spell
surface reprojects the fine grid onto the replacement base surface.

## Aging, clearing and residency

The existing X_eTaL wind exposure fades pressure strength in eight levels, so
geometry gradually fills as the print fades. Quantization avoids rebuilding every
tile for every tiny opacity change. This is geometric recovery from the existing
wind model, not physical wind transport or deposited snowfall.

Clear tracks clears the pressure stamps and removes fine impressions. Reset also
clears spell edits. The source history remains bounded at 256 prints. Leaving the
local field evicts buffers; revisiting reconstructs tiles from the surviving
source history. Prints that age out or leave that history are not saved. Durable
snow storage and weather simulation remain later milestones.

## Budgets and reproducibility

A default tile has 21 by 21 vertices and 800 triangles. A 6.25 cm tile has 33 by
33 vertices and 2,048 triangles. At the 36-tile cap, default fine row/height payloads
occupy about 248 KiB, or about 613 KiB at 6.25 cm. Staging can temporarily retain
another set. The composed mesh, coarse spatial index, CPU triangle views, GPU
buffers and X_eTaL temporaries are additional allocations. Reserve 16 MiB for
these layer resources; this is a design budget, not a measured process-memory cap.
The data-snow-bytes diagnostic counts retained grid payloads and the coarse mesh,
not the entire process, renderer or pending allocations.

The default maximum adds 28,728 triangles after replacing coarse cells. Static
fine grids are indexed, and unchanged tiles retain their displacement buffers.
One tile job per frame limits batch size, not elapsed CPU time. The initial target
is less than 8 ms of native layer/update work at p95; browser FPS must be measured
separately. New source inputs can supersede pending work before it is committed.

Run python3 scripts/build-demo.py, then use scripts/check-demo.py --performance
on a host permitting local sockets and WebGPU. The full scripts/gate.sh discovers
the snow workspace. Cross-tile replay tests check depression, rims, shared border
samples, indexed render/collision agreement, clearing and repeatability.
The snow_cost native example measures walking and resting Scene updates, including
streaming, pressure work and wind updates. It excludes GPU and audio work.

## Native acceptance evidence

The full gate passes 101 Rust tests and 18 Python tests, plus formatting, lint,
shader and WASM checks. Tests also cover 6.25 cm negative-coordinate border replay,
invalid spacing, pending snapshots, stale-work cancellation, eviction and revisit.
Actual browser appearance, audiovisual behavior and FPS remain unverified here
because the sandbox prevents the browser harness's local socket setup.

A native release snow_cost sample measured walking at 4.336 ms median, 6.208 ms
p95 and 12.804 ms worst. Resting, including quantized wind recovery, measured
0.627 ms median, 7.943 ms p95 and 10.056 ms worst. The sample ended with six fine
tiles, 656,736 retained payload bytes and 17,588 terrain triangles. This meets the
initial native p95 target on this host; it is not a browser FPS measurement.
