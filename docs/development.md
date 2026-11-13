# Development process

Read `AGENTS.md` and `CLAUDE.md` first. They adopt the X_eTaL repository's
agentrail protocol, component architecture, test-first development and quality
rules. Use `agentrail next`, read its prompt, then `agentrail begin` before work.
Run the gate, commit source and saga metadata, complete the step, persist the
completion metadata in a separate bookkeeping commit, then stop. Do not start
the next implementation step in the same handoff.

## Local checks

- `bash scripts/gate.sh`: archive hashes, ASCII documentation, American spelling,
  component fmt, clippy, tests and Software Wrighter checklist.
- `python3 scripts/inspect-xetal.py`: optional read-only research snapshot checks.
- `agentrail status`: durable step state.
- `agentrail audit`: saga and git history comparison.

The bootstrap facades have no behavior and therefore no application tests yet.
New behavior needs a failing test, implementation, refactor and relevant gate.
Tests belong in integration-test files by concern. CLI output uses reg-rs when
an application CLI exists; no CLI or baseline is fabricated for empty facades.

The gate excludes preserved upstream files and append-only saga records from
owned-source spelling and ASCII checks. The generated agentrail briefing is
excluded from ASCII checking; the owned instructions remain checked. The gate
verifies archive integrity instead of rewriting archived files.
Rust code follows the stricter design targets: 25 lines per function, five
functions per module, five modules per crate and five crates per component.

## Dependency reproduction

`sources.lock.json` identifies the inspected repository commits and their URLs.
The work copies are ignored and used read-only by convention. They were copied
from local cached refs after DNS prevented a remote fetch; freshness is unverified.
Run `python3 scripts/inspect-xetal.py` to check hashes and each sibling's own
`XETAL_COMMIT`. The independent pins differ and must not be overwritten blindly.

A clean build of the new facades needs Rust/Cargo and no external crates. The
source-baseline step will pin the browser toolchain and add a network-enabled
dependency bootstrap that verifies exact revisions and builds the existing WASM
evaluator before integrating it. No clean-checkout browser build is claimed yet.

## Evidence and publication

Record seed, fixed simulation ticks, input replay, dependency commits, toolchain,
browser, GPU, resolution and quality settings. Compare evaluator, XIR reference
and implemented GPU backends with declared numerical tolerances. Replay edits
and collection exactly; use tolerant image comparisons for GPU output.

Measure CPU frame percentiles, optional GPU timings, active chunks, memory,
queued jobs and upload volume. Validate WASM and worker paths under the Pages
repository subdirectory. Publication requires a real built artifact and must be
reported separately from local checks. Record push failures and unpushed branch
state in the handoff; never retry authentication or DNS failures in a loop.
