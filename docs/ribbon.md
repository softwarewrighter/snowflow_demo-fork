# Ribbon

Enter the demo, hold **2**, and turn with the arrow keys to steer a stream of
water. Release 2 to throw the head along the captured aim. The tail follows its
recorded path and retires. The Ribbon button starts a latched hold; click it
again to throw. Losing window focus releases the hold. Pause freezes time;
Reset clears the effect and terrain edits.

[Ribbon.xtl](../components/world/libs/Ribbon.xtl) owns spring-driven tip motion,
figure-eight aim offsets, inertial steering after release, path history, retirement,
ground response and scoring. It retains at most 32 timestamped tip samples.
Held poses come from the host, but a released stream keeps its release aim.
Recasting starts a new path at the latest pose. Ground facts come from the
current committed collision mesh.

[RibbonSurface](../components/world/libs/RibbonSurface.xtl) generates flattened,
twisting rings along an arbitrary 3D spine, using stable perpendicular frames.
The body has at most 496 triangles. Rust supplies checked typed mesh transport
and a bounded 31,104-byte water buffer, with one additional draw call. The
water material uses procedural highlights and opacity; it does not yet refract
the background or reproduce the original transported-frame/material fidelity.

[RibbonBrush](../components/world/libs/RibbonBrush.xtl) uses the shared signed
TerrainBrush spot profile for shallow ground scores, retaining the latest sixteen
stamps. New contact stamps require movement and a minimum interval. The stream
head drains on impact after release. Exact narrow grooves are limited by the
two-meter base grid; see [the snow layer roadmap](snow-mechanics.md). Spray,
release droplets and a dedicated stream sound are not implemented.

The [composition program](../components/world/demos/ribbon.xtl) is displayed live
as typed and as rendered, with links to actual library APIs and sources. The
`xetal-world-ribbon` crate provides native input and ground-probe transport.
Existing queued Event messages carry a few input scalars; typed arrays carry
output. This remains an application adapter, not the pending generic E1 host API.

Native tests cover hold/aim history, frozen release aim, flight and retirement,
contact scoring, reset, replay/recast, resource bounds and shared edited terrain.
Run the regular post-publication gate and the browser harness for actual device
rendering, held-key/button input, focus loss and FPS acceptance.

## Native CPU sample

A release native Scene sample measured held updates at median 2.049 ms and
p95 3.109 ms. The four-second release/drain/settled phase measured median
0.613 ms, p95 2.150 ms and worst 9.316 ms, including one terrain score. These
samples exclude GPU, audio and browser frame pacing, and contain no footprints.
They are not browser FPS measurements. See [raw evidence](evidence/ribbon.json)
and the reproducible `ribbon_cost` example in the app crate.
