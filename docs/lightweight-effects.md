# Lightweight materials, physics and shadows

The preview adds three independent building blocks.

| Feature | Implementation | Bounded cost |
| --- | --- | --- |
| Material detail | Procedural snow grain, wind bands and ice striations in the fragment shader | No image assets or extra draw calls; a few arithmetic/sine operations per pixel, with grain faded by pixel footprint |
| Player physics | Pure vertical-motion function with gravity, jump impulse and terrain contact, scheduled at 120 Hz | At most 12 catch-up steps per displayed frame; one existing triangle-floor query per frame |
| Sun shadows | One 512 by 512 Depth32Float map, one shared orthographic light projection, comparison filtering | 1 MiB depth allocation; one extra draw of up to 12,800 triangles when geometry or the local light origin changes, one comparison lookup per visible fragment |

Use Space to jump. Holding Space does not repeatedly jump, and a second jump
in midair is ignored. Pause freezes motion. WASD and arrow-key controls remain.
Textures and Shadows buttons independently disable their effects. Turning
shadows off skips both depth-map updates and fragment shadow lookups; turning
textures off skips procedural detail. Depth resources stay allocated so
toggling does not allocate every time.

Terrain is stationary. The shadow map is cached until chunks change or the
local shadow origin moves to follow the player on a 16-meter grid. The sun
direction is fixed, and projection code is shared by both rendering passes.
Depth bias reduces self-shadow artifacts; the small map trades detail for cost.
This is a single directional light, without cascades, screen-space ambient
occlusion, reflections, texture downloads or a general rigid-body engine.

## Ownership and pure state

The X_eTaL world reducer and its single publication-slot exception are unchanged.
The Rust Motion update consumes a small state value and returns its replacement.
Mutable scheduling/input/GPU ownership stays at the host boundary. Gravity and
contact are currently part of the Rust navigation adapter, not an X_eTaL physics
library; porting this pure update to a reusable X_eTaL library remains future
work once navigation state is integrated with the language event interface.

The xetal-world-shadow crate owns cached GPU depth resources separately from
the visible renderer. It is reusable rendering infrastructure, not a new
language primitive. Procedural material shader code belongs to this renderer
until the planned X_eTaL-to-WGSL path can express graphics-stage material work.

## Validation and FPS budget

Native tests cover jump/landing, no midair jump, gravity consistency, raised
terrain contact and queued input across frames shorter than one physics step.
Both the depth shader and visible shader validate against Naga's baseline
capabilities. The application also passes WASM compilation and lint checks.
These checks do not establish browser pipeline correctness or visual quality.

On a normal WebGPU host, build and run:

```sh
python3 scripts/build-demo.py
python3 scripts/check-demo.py --performance
```

The harness checks jump and visual toggles, preserves earlier render/source/
reset/error checks and measures three seconds of animation-frame intervals
with effects off, textures only, shadows only and both. The result includes
mean and 95th-percentile intervals, FPS and frames longer than 25 ms. Each mode walks
the same three-second path from reset across a chunk boundary. Compare the same device, browser, viewport
and pixel ratio. Animation-frame intervals are not GPU timestamp measurements;
headless results may differ from the user's visible browser.

A practical acceptance target is less than 10% increase in the 95th-percentile
frame interval against effects off, at the same scene and settings. This is a
target, not an achieved result. The agent sandbox denies local sockets, so the
hardware browser harness and frame-time comparison remain pending. No measured
FPS claim is made for this preview. Use the independent toggles if a device
misses its frame budget; measure before raising shadow resolution or adding
more lights or rigid bodies.
