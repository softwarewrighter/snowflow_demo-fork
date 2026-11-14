# Crystallise

Press 4 or the Crystallise button after entering the demo. Twelve tapered
hexagonal prisms grow in a golden-angle spiral four meters ahead of the player.
Each crystal starts slightly later; growth finishes within 1.8 simulated
seconds. The latest four casts persist, replacing the oldest on the fifth cast.
Pause freezes growth. Reset clears formations; Clear tracks preserves them.

[Library and native backend reference](libraries.md#crystallise-spell) identifies
actual APIs, source, ownership and tests. The live source panels show the
separate spell composition program with syntax colors and clickable library
calls. All geometry is procedural. No model, texture, handwritten JavaScript
or handwritten WASM asset is required.

## Budgets and limitations

The renderer uses one additional draw call with at most 48 crystals and 864
triangles. Its fixed vertex buffer is 31,104 bytes. X_eTaL computes growth at
10 Hz; settled formations reuse their existing mesh and do not upload again.
The preview now uses Trunk release builds to optimize Rust/WASM execution.

A release native run of the reproducible `cost` example measured four active
formations at 4.788 ms per growth update and 0.055 ms per settled update on this
host. These means describe CPU evaluator/transport work, not browser FPS or GPU
time. Browser performance needs measurement on the target device.

The first port uses a faceted cyan material with alpha blending and depth
writes. Refraction, original spiral/prism detail parity, ice collision, spell
lights, crystal shadows and sound are not implemented. Bases use the procedural
terrain field and are buried slightly to meet the coarser rendered surface;
steep slopes may still expose a mismatch. Positions use the current f32 renderer.

## Verification

Native tests cover deterministic growth, aim, geometric height, four-cast bounds,
reset payloads, paused growth, finite/rejected inputs and shared-camera WGSL
validation. The browser harness casts with key 4, captures a crystal screenshot
and verifies reset. This sandbox denies local sockets, so actual rendering,
appearance, audio and FPS acceptance remain pending on a normal host.
