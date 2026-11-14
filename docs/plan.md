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
| 5 | browser-mesh-demo | Continuous X_eTaL-driven mesh, Rust renderer, input and error handling in a real browser |
| 6 | workers-and-replay | Bounded jobs, versioned transfers, cancellation, deterministic fixed-step replay |
| 7 | pages-foundation | Static subdirectory build, working WASM/worker paths and reproducible browser evidence |
| 8 | smooth-snow-chunks | Endless smooth terrain, snow and ice boulders with matching borders and bounded residency |
| 9 | digging-and-gems | Spherical edits expose dirt and gems; collision, save/load and collection agree |
| 10 | flowing-water | Lake breach flows over edited terrain with connected water surfaces and explicit source rules |
| 11 | torches-and-gem-hunt | Local lights, day/night and fixed-tick gem hunt over the smooth world |
| 12 | foundations-review | Clean-checkout replay and browser evidence; reusable features and upstream asks reviewed |

Work reusable across applications belongs in the appropriate X_eTaL repository,
then reaches this app through explicit pin updates. Initial component facades
are scaffolding, not substitute implementations of missing language features.

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
Neither the roadmap nor empty Rust facades constitute a delivered browser demo.

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
bounded resource ownership. They compile for native and WASM targets. No renderer,
GPU upload or evaluator host hook is claimed. Existing E1 remains the upstream
integration requirement. Native payload timings describe serialization stages
only; see [mesh contract](mesh-contract.md) and [host binding requirements](xetal-asks.md).
