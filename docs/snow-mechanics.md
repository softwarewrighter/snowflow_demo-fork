# Snow mechanics roadmap

This is a proposed design for an RDR2-inspired interaction goal. It is not an
account of Rockstar's private implementation. Rockstar's
[PC announcement](https://www.rockstargames.com/newswire/article/398a45525ka319/Red-Dead-Redemption-2-For-PC-Now-Available-to-Pre-Purchase-via-the-Roc)
identifies improved snow trails as a visual feature. The design below describes
what this demo can build with X_eTaL and Rust.

## Current limits

Boot prints now use the [fine geometric layer](deformable-snow.md), with
X_eTaL boot depressions, raised rims and quantized wind-driven recovery. Sweep, Bloom
and Ribbon perform real height edits, but the base grid is two meters wide per
cell and retains bounded spell descriptors. The separate near-player layer represents boot-scale impressions; persistent
deep-snow trails remain future work. Snow currently has no loose-depth,
compaction or density state, and walking does not depend on snow depth.

## First mechanical milestone

Add a separate fine snow layer above the coarse streamed base terrain. A
proposed initial setting is a 256 by 256 near-player field at 6.25 cm spacing,
covering a 16-meter square. A 128 by 128 option reduces cost. These are proposed
starting budgets, not measured or implemented settings.

Four f32 channels per 256-square field occupy 1 MiB; two buffers occupy 2 MiB,
before normals, meshes, tile caches and transfer staging. Rendering should reuse
static grids and update dirty regions. Subdividing the entire endless world to
this density or rebuilding every chunk per footstep would be too costly.

Each sample can hold `(surfaceHeight, looseDepth, compaction, wetness)` above an
immutable hard base. Procedural pressure stamps create boot-shaped depressions,
raised powder edges, landing pits and continuous drag marks. Use the same
committed field/version for visible displacement and collision sampling.

Acceptance: walk a short trail, turn around and see actual depth and edges;
repeat a step and approach the hard/packed layer rather than cutting forever;
cross a tile boundary without a seam. Record fixed input replay, memory, update
cost and browser frame percentiles. A first-person view can demonstrate this
before animated legs or a third-person character exist.

## Compaction and movement

Fresh powder should compress more than an already-packed track. Pressure,
contact area, landing speed and horizontal motion affect indentation. Convert
some loose snow to denser packed material and displace a bounded fraction into
neighboring cells. Account for mass using depth and density; compaction can
reduce volume without deleting the corresponding mass. Rims should come from
those accounted transfers instead of adding unlimited material.

Sink depth and resistance should affect walking and jumping. Reusing a packed
trail should reduce sinking and drag. Crunch/thud gain, pitch and variation
should derive from depth, compaction and speed. Keep camera motion tied to
physical contact; do not reintroduce idle ground bobbing. Animated feet, leg IK
and knee/body drag are later presentation/contact work, not prerequisites for
the first geometric-footprint milestone.

## Weather and persistence

Wind redistributes loose surface snow, smoothing edges and filling depressions.
Snowfall adds new loose snow. Packed tracks can persist longer than loose rims.
This must change geometry and material state, replacing opacity-only recovery.
Sparse modified tiles can live in a bounded resident cache and persist through
Rust browser storage. Evicting a tile from RAM should not erase its saved trail.
Keep world seed, tile coordinates, schema and edit epoch in the saved record.

Use versioned worker jobs, reject obsolete outputs and commit render/collision
state together. Avoid synchronous GPU readback in the walking path. GPU/worker
kernels need reference-oracle and tolerance tests against the pure library rules.

## Ownership and phased delivery

| Proposed facility | Owner | Responsibility |
| --- | --- | --- |
| SnowLayer library | X_eTaL | Pure stamp, material and compaction rules over immutable tile values |
| SnowWeather library | X_eTaL | Wind transport, settling, snowfall and explicit mass sources/sinks |
| Contact and sound policy | X_eTaL | Depth/pressure response, movement resistance and sound choices |
| Snow tile backend | Rust extension | Bounded buffers, spatial lookup, dirty uploads and worker execution |
| Snow tile storage | Rust extension | Browser persistence, versioning and resource lifecycle |
| Rendering/animation backend | Rust extension | Displacement, normals, shadows and eventual character/IK capability |

SnowLayer now exists in the live catalog; SnowWeather, material compaction and
durable tile storage remain planned facilities. Extend actual TerrainBrush and SnowParticles primitives where suitable.
Generic typed host bindings and faster array execution remain forcing functions;
do not hide the workload by moving application policy into Rust.

After the five-spell port, agentrail schedules: deformable-snow-layer,
snow-compaction-contact, then snow-weather-persistence. Publish each visible
milestone before the full gate, then fix regressions. Start with geometric boot
prints; full cinematic character interaction and material fidelity are larger
follow-on work. See [the implementation plan](plan.md).
