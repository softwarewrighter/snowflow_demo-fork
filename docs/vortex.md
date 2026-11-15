# Vortex

Press 5 or click Vortex. Three rotating snow/slush helices surround the player,
with short-lived grains launched tangentially from those paths. A shallow ring of
surface snow lifts into the effect and settles back. Recasting restarts the spell;
pause freezes it and reset clears its geometry, particles and temporary lift.

## X_eTaL building blocks

[App.xtl](../components/world/demos/App.xtl) configures the fifth binding.
[Spells.xtl](../components/world/libs/Spells.xtl) imports Vortex.xtl and dispatches
it. No native spell constructor or key dispatcher was added.

[Vortex.xtl](../components/world/libs/Vortex.xtl) owns immutable lifecycle,
rotation, helix shape, deterministic particle emission and lift strength. It uses
RibbonSurface for three 33-sample tubes and SnowParticles for ballistic grains.
The envelope ramps over 0.55 seconds, holds for 3 seconds and fades over 1.1
seconds. Particle tails finish by 5.25 seconds. The column reaches about 4.8
meters above its ground estimate and follows the current player pose.

[VortexBrush.xtl](../components/world/libs/VortexBrush.xtl) supplies a signed
annular profile with up to 0.32 meters of temporary displacement.
[World.xtl](../components/world/libs/World.xtl) tracks the current lift and marks
both old and new regions dirty when it moves. SnowTerrain adds this profile to
the other signed brushes. Zero lift restores the underlying edited terrain,
rather than resetting other spells' edits. Rendered and collision terrain use
the same committed chunk geometry. The ground estimate uses the supplied player
eye height; it is not a separate fine-resolution terrain probe.

## Budgets and rendering

The three helices use 1,536 triangles. Sweep and Bloom can coexist with Vortex
in the slush material slot: the aggregate limit is 2,400 triangles. Ice and water
remain capped at 864 triangles each. The reusable native geometry buffer is
large enough for the slush slot; native code contains no Vortex lifecycle logic.

Vortex uses a deterministic 512-slot analytic particle pool. Bloom can contribute
another 512 rows, staying within the existing 1,024-particle backend limit.
Each grain launches tangent to a helix and lives at most 0.56 seconds. The pool
recycles analytically with staggered births, rather than keeping mutable native
emitter state. Particle rows and geometry reuse the existing native renderers.

## Original-demo parity limits

This preserves the original's three-strand silhouette, approximate envelope,
rotation, tangential grains and reversible surface lift. It is a first procedural
port, not full visual parity. The original uses 64 spine samples, more grains,
transported circular frames, richer material relief and a local light. This
version uses 33 samples, the existing elliptical tube/slush material and no new
light. Particle birth angles approximate recent spin and use the current player
center; fast movement does not leave a fully simulated grain trail.

Surface lift is one smooth annular displacement that follows the player and
restores old locations. The original accumulates rotating small brushes and
loose deposited snow. This version does not model conserved snow volume,
compaction or physically deposited grains. The 2-meter terrain grid limits fine
surface detail. Finer snow interaction remains a separate development phase.

## Reproduce

Run scripts/gate.sh for native, lint, shader and WASM checks. Session tests cover
key 5, lifecycle, combined effect budgets, reset and exact terrain restoration.
Use python3 scripts/build-demo.py to rebuild the preview. The generated library
reference and the Vortex source panel show the actual library and public API.
Actual appearance, sound interaction and FPS require the browser harness and
human inspection on a host permitting WebGPU and local sockets.
