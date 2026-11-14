# Functional world state

The demo passes an immutable (phase, boost) tuple to its event reducer.
Height and terrain generation are pure. A frame function returns the next
logical state and derived mesh, reusing the old mesh for unchanged state.
The effect loop publishes that mesh, receives an event and recurs with the
next pair. Only the typed host publication slot mesh! remains mutable.
It is never read by the reducer or geometry functions. Rust platform/GPU
resource mutation is outside these world-state functions.

## Verification

The prior program is preserved verbatim as the snowfield-mutable.xtl test
fixture from source commit 1ed8f7b. Native replay compares every position and
index exactly after ticks, down events, ignored choose events, reset and end.
Existing tests still cover scheduling slices, independent instances, geometry
winding and invalid payloads. AST checks enforce the single publication-slot
exception and prevent reintroducing mutation into pure functions.

The world integration tests run the full upstream lint driver with default
levels and reject enabled findings. A control test confirms it finds the old
unused changed binding. This is project enforcement in addition to existing
upstream lints; no new language rule or upstream lint is claimed.

## Reproduce the comparison

After dependency bootstrap and cache preparation described in development.md:

```sh
CARGO_HOME="$PWD/work/cargo-home" cargo run --offline --release --manifest-path components/world/Cargo.toml --example compare_state
CARGO_HOME="$PWD/work/cargo-home" cargo run --offline --release --manifest-path components/world/Cargo.toml --example compare_state -- ticks
```

Both variants run in the same executable using the same pinned evaluator and
adapter. Each variant has ten samples of 64 events; order alternates each round.
Timed work includes event feed, evaluation in 8,000-transition slices and typed
mesh conversion. Compilation, loading and initial geometry are excluded. A
separate one-transition-per-slice pass counts evaluator work deterministically.
The mixed trace uses tick 0.08 except every 16-event block: reset at 0, down at
7 and ignored choose at 15. The other trace contains only ticks.

Measured on an Intel Xeon W-2135 with Rust 1.100.0, release profile:

| Trace | Before median per event | After median per event | Time change | Evaluator transitions |
| --- | --- | --- | --- | --- |
| Mixed | 3.608 ms | 3.405 ms | -5.6% | 38,304 to 43,193 (+12.8%) |
| Ticks only | 3.804 ms | 3.803 ms | about 0% | 40,832 to 46,016 (+12.7%) |

There is a real negative cost: tuple handling and pure function calls increase
evaluator transitions by about 13%. The measured tick wall time does not show
a discernible slowdown: the distributions overlap on this shared machine. The
mixed trace benefits from avoiding unnecessary mesh generation on unchanged
state. An initial uncached refactor regenerated terrain on ignored events and
was slower; the shipped frame function avoids that work.

These are native evaluator and adapter measurements, not browser FPS results.
GPU time, WASM timing, memory use and worker scheduling were not measured. The
raw samples and source hashes are in docs/evidence/immutable-state-performance.json.
Actual browser render/input acceptance still requires scripts/check-demo.py on
a WebGPU host; local socket creation is denied in the agent sandbox.
