# Implementation plan

Development uses agentrail sagas. `.agentrail/` is the durable active record;
completed or superseded sagas move through `agentrail archive`. Update saga state
only with agentrail commands. Every implementation step is test-first and ends
with the gate, a code/metadata commit, `agentrail complete`, and a bookkeeping
commit of the completion record. Stop after completing the active step.

## Saga 1 snowy world foundations

Active saga: `snowy-world-foundations`. The bootstrap step establishes repository
structure; completed work and pending validation are recorded below and in agentrail.

| Step | Slug | Acceptance result |
| --- | --- | --- |
| 1 | repository-bootstrap | Original demo archived intact, new README, legal notices, instructions, components, docs and quality gate |
| 2 | source-baseline | Tested dependency bootstrap, native evaluator/runner checks and WASM build; browser harness supplied |
| 3 | typed-mesh-contract | Validated application payloads, checked adapters, bounded resources and existing E1 requirements |
| 4 | browser-baseline-validation | Real-browser editor and event-worker evidence before the browser mesh demo |
| 5 | pages-foundation | Publish the built static artifact with trusted HTTPS and verify asset URLs |
| 6 | streaming-tracks-audio | Stationary terrain, bounded endless streaming, footprints and gesture-enabled movement/landing sound |
| 7 | lightweight-effects | Procedural material detail, fixed-step jumping and cached sun shadows with cost controls |
| 8 | browser-mesh-demo | Continuous X_eTaL-driven mesh, Rust renderer, input and error handling in a real browser |
| 9 | workers-and-replay | Bounded jobs, versioned transfers, cancellation, deterministic fixed-step replay |
| 10 | smooth-snow-chunks | Endless smooth terrain, snow and ice boulders with matching borders and bounded residency |
| 11 | digging-and-gems | Spherical edits expose dirt and gems; collision, save/load and collection agree |
| 12 | flowing-water | Lake breach flows over edited terrain with connected water surfaces and explicit source rules |
| 13 | torches-and-gem-hunt | Local lights, day/night and fixed-tick gem hunt over the smooth world |
| 14 | foundations-review | Clean-checkout replay and browser evidence; reusable features and upstream asks reviewed |

Work reusable across applications belongs in the appropriate X_eTaL repository,
then reaches this app through explicit pin updates. Application adapters are not substitutes for missing language features;
generic changes still belong upstream.

## Saga 2 visual fidelity

Planned successor, to initialize after the foundations review. Keep one active
saga rather than creating competing active state files.

1. Capture a fixed-camera visual baseline and hardware-specific frame budget.
2. Add snow material detail, coverage, exposed dirt and rock.
3. Add atmosphere, sky and shadows with matching displaced geometry.
4. Improve ice boulders, water lighting, refraction and falling streams.
5. Add torch flames, particles and local-light effects.
6. Add temporal/post-processing effects individually with measured cost.
7. Review against the archived Snowflow reference and publish reproducible evidence.

## Saga 3 browser GPU compilation

Planned successor or explicitly scheduled independent lane if measurements make
acceleration urgent. It is not a prerequisite for the first playable world.

1. Define the supported numeric and array subset with evaluator oracles.
2. Implement XIR-to-WGSL emission and browser compute execution.
3. Verify maps, lookup/rotate, reductions and required structured regions.
4. Keep arrays resident and share buffers with the renderer under one device owner.
5. Move measured terrain/water workloads to compiled X_eTaL kernels.
6. Specify graphics-stage/texture operations before attempting general shader generation.
7. Compare supported backends, refusals, precision and performance in repeatable runs.

## Detailed design and evidence

[Architecture](architecture.md), [decisions](design.md) and
[development](development.md) govern application work. The detailed initial
[world roadmap](xetal-world.md) provides representation and ownership analysis.
The local application now has a procedural mesh build; browser render acceptance
is recorded separately from the earlier upstream editor baseline.

## Source baseline findings

The dependency bootstrap verifies every inspected commit and each sibling core pin.
The evaluator and runner native suites and their WASM build are checked separately
from browser execution. This sandbox denies loopback socket creation, so browser
execution was kept separate while the native mesh contract proceeded.
A terminal-host result now verifies the upstream editor and event worker. Raw
messages, browser capabilities and matching WASM hashes are committed as evidence;
this validates the existing evaluator path, not a new application renderer. See
[source baseline](source-baseline.md) for commands, tools and evidence limits.

## Typed mesh contract evidence

The application surface and host crates implement checked payload conversion and
bounded resource ownership. They compile for native and WASM targets. The first
application now adds GPU upload and rendering through an application-specific
evaluator capture adapter; generic typed host registration is not implemented. Existing E1 remains the upstream
integration requirement. Native payload timings describe serialization stages
only; see [mesh contract](mesh-contract.md) and [host binding requirements](xetal-asks.md).

## First browser mesh

The application program generates a continuous 128-triangle patch in X_eTaL.
A persistent sliced machine handles tick, pointer and reset events. Typed values
feed the validated surface adapter and wgpu renderer without numeric text transfer.
The local launcher serves the compiled artifact at port 8475 under the repository
URL prefix. Native event oracles, shader validation and WASM checks are separate
from the actual browser rendering/input/reset/error-feedback acceptance. Worker
scheduling remains the next implementation step after this acceptance passes.

The first local artifact is built and all native, shader and WASM checks pass.
Application browser acceptance is pending a terminal-host report and screenshot;
the earlier upstream-editor validation does not satisfy this step. The active
step remains in progress until those actual rendering and interaction checks pass.

Pages publication was promoted ahead of browser acceptance at the user request
so the demo can be opened over trusted HTTPS. HTTP asset checks and publication
are distinct from hardware rendering/input acceptance, which stays open.

The gh-pages branch is now published at
https://softwarewrighter.github.io/snowflow_demo-fork/ with HTTPS enforced.
GitHub reports built. The index and all five static assets return HTTP 200 with
matching build hashes; the WASM response uses application/wasm. The deployment
record is docs/evidence/pages.json. Browser hardware rendering acceptance remains
the next step and is not inferred from these HTTP checks.

## Walkable preview checkpoint

Step 6 now includes the user-requested finite walkable snowfield before workers.
The terrain expands to 2,048 X_eTaL-generated triangles, with snow-covered hills
and two rounded ice formations. Rust host navigation follows the actual triangle
heights. Native movement tests, shader validation and WASM builds are prerequisites
for publication; actual browser movement/reset acceptance remains a separate check.
Endless chunks, digging, gems, water and torches retain their later steps.

## Functional state checkpoint

The step 6 world program now threads immutable logical state through pure update
and geometry functions. One mesh publication slot remains at the host boundary,
as permitted by the user. Exact legacy/new replay, upstream linting and an AST
mutation guard run in the native gate. Repeatable native measurements record
roughly 13% more evaluator transitions; tick wall-time distributions overlap.
See functional-state.md and its raw evidence for costs and measurement limits.

## Lightweight effects checkpoint

The user-requested effects work preempts browser acceptance as step 6; the
previous mesh acceptance step is now step 7. Native physics and shader tests
remain separate from real-browser visual and frame-time acceptance. The world
reducer stays immutable. Rust supplies a pure vertical-motion update through
the existing navigation adapter and GPU resources through a small shadow crate.
See lightweight-effects.md for defaults, toggles, costs and measurement commands.

## Stationary streamed world checkpoint

The latest user requests preempt effects acceptance as step 6. The source
now generates deterministic stationary world-space chunks. A 25-chunk window
follows movement past the original arena, with matching edges and fog masking
the distant residency boundary. Bounded footprint decals and gesture-created
procedural WebAudio share grounded footfall events; landing has a separate
thud. Existing worker/replay and higher-detail chunk milestones remain ahead.
See streaming-snow.md for current bounds, measurements and browser acceptance.
