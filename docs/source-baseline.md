# Source baseline reproduction

The dependency bootstrap creates disposable build checkouts under `work/deps/`.
It verifies full commit hashes and each sibling's independent core pin from
`sources.lock.json`. Existing dirty checkouts are rejected without discarding
changes. Research checkouts under `work/X_eTaL*` are only read as Git sources.

## Bootstrap

With network access, run `python3 scripts/bootstrap-xetal.py`. With the inspected
local repositories already available, run:

```sh
python3 scripts/bootstrap-xetal.py --cache-dir work --offline
python3 scripts/inspect-xetal.py
```

A local cache can also be another directory holding Git repositories with these
names. The cache must contain the exact application pins and the core commits
named by each sibling's XETAL_COMMIT. No remote freshness is inferred from a cache.
The bootstrap is covered by tests for exact revisions, repeated execution,
preservation of local edits and missing offline commits.

## Build the evaluator and worker

Select the versions recorded in `tools.lock.json`, including Rust 1.100.0,
Trunk 0.21.14 and wasm-bindgen 0.2.129. Install the wasm32-unknown-unknown target
and populate the Cargo cache before using the offline build. All application
components use Rust edition 2024; crate manifests inherit their workspace edition.

Run `python3 scripts/build-baseline.py`. It checks tool versions, runs the
upstream xetal-play and xetal-runner native suites, then builds the existing
editor and worker with locked Cargo dependencies. Output is under
`work/baseline-site/snowflow_demo-fork/`. This builds the upstream evaluator,
not the snowy world app, and does not publish a site.
Successful builds write `work/baseline-evidence.json` with versions, source
revision and artifact sizes/hashes. Browser execution is explicitly separate.

When several wasm-bindgen versions are installed, select the matching CLI on
PATH. For this machine the command was:

```sh
PATH=/home/mike/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:/home/mike/.cache/trunk/wasm-bindgen-0.2.129:$PATH python3 scripts/build-baseline.py
```

The direct toolchain path avoids a rustup self-update cleanup attempt against a
read-only Cargo bin directory in this sandbox. It does not change the selected
compiler version. The script normalizes NO_COLOR to the boolean value accepted
by this Trunk version. Neither the PATH adjustment nor environment normalization
changes upstream sources.
The repository gate calls cargo-fmt and cargo-clippy directly so PATH selects
these same toolchain binaries rather than stale proxies in CARGO_HOME/bin.

## Browser execution

Run `python3 scripts/check-browser-baseline.py` on a host permitting local server
sockets and headless Chromium. It uses Python's standard library and ChromeDriver,
with no Selenium installation. Chromium and ChromeDriver must have compatible
versions; the observed versions are recorded in `tools.lock.json`.

The smoke test serves the built artifact at the repository subdirectory, checks
that the existing editor mounts, then starts the generated worker loader. It
expects Ready, an event wait from `[]E_VENT`, a supplied `tick 0.016`, the output
`tick`, and Done. The worker is terminated after the check. The JavaScript in
this test drives the browser protocol only; it is not application code.
The four messages must arrive in that order, without worker errors. Successful
results include actual browser capabilities and console logs. Both successful
and failed runs save `work/browser-baseline-result.json`; a failed attempt
replaces an earlier success so stale results cannot be mistaken for a new pass.

This environment denied socket creation before the browser test could start:
`PermissionError: [Errno 1] Operation not permitted`. Separate Chromium attempts
also encountered crash-handler/socket restrictions. These are failed environment
preflights, not successful browser results. Browser mount, worker loading and
event delivery remain unverified until the explicit validation step passes.

## Reproducibility limits

Pinned commits and Cargo.lock files identify source inputs; tools.lock.json
identifies observed tool versions. No package installation is implied by the
version checks. A network-free clean build also requires a populated Cargo and
Trunk tool cache. Generated worker cache IDs contain build time, so identical
binary hashes across rebuilds are not promised. Compare functional results.

The upstream sources remain unchanged. Their existing UI and service-worker
code include JavaScript; this baseline is compatibility evidence and does not
make those files part of the new application's implementation.
