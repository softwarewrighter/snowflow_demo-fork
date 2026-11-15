# Single application composition

The live application is [App.xtl](../components/world/demos/App.xtl). It imports
[Spells.xtl](../components/world/libs/Spells.xtl), which selects the five existing
spell libraries and dispatches configured input bindings. The native application
contains one Session, no per-spell constructors or key dispatcher. Its production
Cargo dependencies no longer include the individual spell runner crates.

## Boundaries

The [source connection guide](source-connections.md) describes the complete path
from input to drawing. The [library cross-reference](libraries.md) links real
modules, public APIs, native capabilities and tests. The generated live reference
includes all imported composition libraries and links from colored source calls.
Controls and source panels come from the typed X_eTaL publication. Buttons retain
numeric order 1, 2, 3, 4, 5. All panels use the existing JuliaMono and syntax palette.

Resource revisions are monotonic across reset. Rust reuses decoded publications
and unchanged mesh buffers. Spells skips mesh generation for unchanged state.
World accumulates dirty bounds until a matching revision acknowledgment. Tiles
builds replacement resident chunks before committing their combined mesh to
rendering and physics; eviction retains the X_eTaL edit histories.

Current capability limits remain explicit: one bounded resource slot per material
(ice, slush, water), 864 triangles for ice/water and 2,400 for slush, 1,024 particles, 256 footprints,
16 physics requests, 25 resident chunks and 64 dirty rectangles. Overflowing dirty
rectangle history conservatively invalidates the resident cache. Brush histories
retain their existing limits. New shaders, query kinds or larger resource budgets
need native backend changes. A new spell using existing capabilities belongs in
X_eTaL composition and must combine geometry within these slots.

Execution remains interpreted on the browser main thread. Scalar Events bridge
input and facts; dense outputs are typed values. Generic E1 host registration,
worker execution, direct numeric GPU compilation and measured browser FPS remain
future work. The archived JavaScript is not part of the active application.

## Verification

The full scripts/gate.sh passes: 102 Rust tests, 18 Python tests, lint, formatting,
shader validation and WASM checks. It discovers the new session workspace.
The suite checks that changing App.xtl's fourth binding to 9 changes actual
Crystallise dispatch without native changes. Standalone programs serve as
independent composition oracles for all four spells, Bloom particles, footsteps
and wind aging. Scene tests cover collision, movement, pause/reset, footprints
and landing audio. Protocol tests reject malformed frames, unsupported versions,
nonfinite navigation, invalid counters/audio, duplicate resources and oversized
geometry. Streaming tests cover current-version acknowledgment and edit retention
across eviction and revisit.

Actual visual/audio/FPS acceptance uses scripts/check-demo.py --performance on a
host permitting local sockets and WebGPU. Sandbox socket restrictions still
prevent that browser check here. Native tests and HTTP hashes do not establish
that browser acceptance.

## CPU sample

Native release samples use the existing Scene examples on this host. They exclude
GPU rendering, WebAudio, browser frame pacing and walking footprints. Prior values
come from the earlier spell documents; these are indicative samples, not a
controlled browser benchmark.

| Workload | Earlier median ms | New median ms | New p95 ms |
| --- | --- | --- | --- |
| Bloom eruption, including crater rebuild | 3.529 | 3.982 | 4.881 |
| Bloom fallout | 2.881 | 3.181 | 3.903 |
| Settled Scene | 0.144 | 0.386 | 0.543 |
| Held Ribbon, with aim changes | 2.049 | 2.176 | 2.484 |
| Released Ribbon including terrain score | 0.613 | 0.753 | 2.339 |
| First Sweep cast | 16.132 | 19.383 | 22.170 |
| Sweep with eight retained cuts | 38.894 | 42.023 | 43.038 |

The composition adds CPU overhead. Settled overhead is about 0.24 ms per sampled
update; active effects and repeated terrain rebuilds cost more. Heavy Sweep
updates still exceed a 60 FPS frame budget. Coalescing application time to about
20 Hz limits repeated work, while drawing and native physics retain browser frame
pacing. Workers and finer dirty-region scheduling remain follow-up work. Do not
infer 60 FPS from these native measurements.

## Fine displacement capability

Protocol version 2 adds source-configured detail tiles. SnowLayer owns boot
pressure, raised edges, overlap bounds and quantized aging. Native tiled buffers
stage one job per update and commit the same indexed geometry and height samples
to rendering and collision. See [fine snow acceptance and budgets](deformable-snow.md).
