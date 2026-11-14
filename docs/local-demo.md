# Local snow surface demo

The first application renders a procedural 2,048-triangle snowfield. X_eTaL
generates its geometry and responds to events; Rust runs the persistent sliced
evaluator, reads typed arrays and submits them to wgpu. There are no model or
texture files. The favicon is a small snowflake SVG matching the preview colors.

## Open the built demo

From the repository root, run:

```sh
python3 scripts/serve-demo.py --no-build
```

Open [the local demo](http://127.0.0.1:8475/snowflow_demo-fork/). The command must
run in a normal terminal that permits local server sockets. By default it binds
to 0.0.0.0 and prints a Network demo URL, preferring the 192.168.x.x interface.
Use that printed address from another machine. Use Ctrl-C to stop the server,
`--port` to select another port, or `--bind 127.0.0.1` for loopback-only serving.

WebGPU needs a secure browser context. For local HTTPS, run:

```sh
python3 scripts/serve-demo.py --no-build --https
```

Open the printed HTTPS address and accept the self-signed certificate warning.
OpenSSL generates a 30-day certificate for the detected LAN IP, localhost and
127.0.0.1. It and its private key are kept in ignored `work/tls/` and reused;
the key has mode 0600. No trust store is changed. Stop the HTTP server before
restarting with HTTPS on the same port. A browser that refuses a certificate
exception needs a trusted development certificate or the GitHub Pages URL.

For LAN HTTP, a development browser can instead be scoped to the preview origin:

```sh
chromium --user-data-dir=/tmp/xetal-lan-preview --unsafely-treat-insecure-origin-as-secure=http://192.168.1.178:8475 http://192.168.1.178:8475/snowflow_demo-fork/
```

Replace the IP with the launcher's printed address. This does not change the
default browser profile. See [Chromium's secure-origin development guidance](https://www.chromium.org/Home/chromium-security/deprecating-powerful-features-on-insecure-origins/).

Click the surface or Change ridge to change its height. Pause stops simulation
ticks while keeping the camera responsive. Reset restores the initial surface.
WASD walks, arrow keys look around, and R resets. Expand the source panel to inspect
the actual X_eTaL program. A supported WebGPU browser and device are required;
an unavailable adapter or device produces status feedback.

## Build from pinned sources

Bootstrap the sources as described in [source baseline](source-baseline.md),
then populate the locked dependency cache in a network-enabled terminal:

```sh
cargo fetch --locked --manifest-path components/app/Cargo.toml
python3 scripts/serve-demo.py
```

The launcher builds before serving unless `--no-build` is supplied. Build alone
with `python3 scripts/build-demo.py`. Static output goes to
`work/demo-site/snowflow_demo-fork/`. The URL prefix is already compatible with
the intended repository Pages path; no deployment is performed by these commands.

The build reuses cached packages through a writable project-local Cargo home,
so extracting new dependencies does not require writing to the shared cache.
New archives must already be cached: the build runs offline, including Cargo
metadata. Trunk metadata may inspect packages for other platforms, so use the
fetch command without a target filter. Tools remain pinned by the recorded
baseline; wasm-bindgen 0.2.129 must match the application lockfile.

## Validate in a browser

Run `python3 scripts/check-demo.py` from a terminal with local sockets and
ChromeDriver. It checks live geometry updates, pause, ridge input, reset and
the unsupported-WebGPU message. It saves `work/demo-browser-result.json` and
`work/demo-screenshot.png`. The screenshot is for visual review; a compiled
WASM artifact alone does not establish that pixels rendered correctly.

The native gate also runs the X_eTaL event oracle and validates the WGSL shader
with Naga. The WASM application is checked with clippy. The headless browser
driver uses WebGPU test flags; the served application does not change browser
security settings. A desktop browser may have different GPU availability from
headless Chromium, so preserve the actual capabilities with each result.

## Current boundaries

This is the first surface, not the endless world. Chunk streaming, digging,
water, torches, gems and worker scheduling are separate milestones. The evaluator
and renderer currently share the main thread; transition slices limit work units,
not elapsed time inside an individual primitive.

The application reads the public evaluator output hook and a captured lexical
mesh binding from the pinned runtime. It avoids numeric text serialization but
still copies and narrows values into GPU buffers. This is an application-specific
adapter, not the generic host-registration feature requested by upstream E1.
The loaded program and keeper are retained for the page lifetime, consistent
with the existing interactive runner. Reset sends an event to that same runtime;
it does not repeatedly parse or leak another program.

The WGSL shader is handwritten GPU code. Browser loader JavaScript and WASM are
generated by the Rust toolchain; there is no handwritten application JavaScript
or WebAssembly. The procedural world algorithm is in
`components/world/demos/snowfield.xtl`.

## Walkable snowfield preview

Use W/A/S/D to walk and the arrow keys to look. R or Reset returns to the
starting position. Pause stops walking and terrain animation; Change ridge
changes the X_eTaL terrain height. The preview covers a finite 32-meter square.
The two ice formations are rounded heightfield mounds with slope-based ice
coloring, not separate overhanging boulder meshes.

X_eTaL produces all 2,048 terrain triangles. Rust supplies the camera and
triangle-interpolated eye height in the small xetal-world-navigation crate.
This is an intermediate application adapter: reusable movement rules and terrain
queries should become an X_eTaL library once the event/state interface is settled.
The Rust extension boundary should retain browser input, GPU resources, workers
and other platform capabilities. There is no hand-authored application JavaScript.

The native movement oracle checks frame-rate independence, bounds and eye
clearance on a sloped triangle. The browser harness additionally checks walking
and reset through the rendered application. Hardware browser acceptance remains
pending until that harness is run successfully on a host with WebGPU.

## Source views

Expand See the X_eTaL program to compare As typed and As rendered side by side
(stacked on narrow screens). The typed pane preserves the shipped ASCII source.
The rendered pane uses xetal-render from the same pinned core checkout, including
its function decorations and mathematical glyphs. Both are selectable text and use JuliaMono Regular 0.63.2 from cdnjs.
The rendered pane uses xetal-view semantic token classes and its HTML formatter,
which escapes source text before wrapping colored spans. The typed pane is plain
text. Highlight colors distinguish functions, numbers, strings and comments.
The font uses swap loading with a local monospace fallback if the CDN is offline.
See https://cormullion.github.io/pages/2020-07-26-JuliaMono/ for upstream web-font
guidance. Source display is initialized before WebGPU checks so it remains
available on unsupported devices.

## Functional state and the publication boundary

The world state is an immutable (phase, boost) tuple. Pure functions update that
state from an event, derive height and terrain, and return (next state, mesh).
The event loop threads that pair through recursive calls. An unchanged logical
state reuses the existing mesh rather than regenerating terrain.

Only mesh! remains mutable: it is a publication slot read by the Rust adapter,
not the source of world-state decisions. The loop named u:l_oop! contains the
input/publication effects; the update and geometry functions have no assignments
to mutable variables. This exception is for the pinned host API, not a claim
that global mutation is faster. Removing the slot requires an upstream typed
publication mechanism usable inside an event loop.

The gate runs the upstream X_eTaL lint driver through a native integration test
and rejects enabled findings. A separate AST check rejects mutable application
state and assignments outside the one mesh publication in the effect loop.
The preserved mutable source is a regression fixture, not the shipped program.
See [functional state performance](functional-state.md) for exact replay tests,
measurements and their limits.

## Optional detail and player physics

Space jumps with gravity and terrain contact. Textures and Shadows buttons
control procedural material detail and one small cached sun shadow map. See
[lightweight effects](lightweight-effects.md) for implementation boundaries,
limits and the scripts/check-demo.py --performance frame comparison. Effects
start enabled, but FPS impact has not been measured in the sandbox.
