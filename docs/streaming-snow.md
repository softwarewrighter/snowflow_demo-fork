# Stationary streamed snow, tracks and sound

Pure X_eTaL world-coordinate functions generate 32-meter chunks with 512
triangles each. A nearest-first 5 by 5 window follows movement, retains at
most 25 chunks and evicts distant ones. Fog hides the residency boundary
at a 60-meter horizon. Movement has no fixed arena wall. Native tests cover
matching edges on both axes, negative coordinates and deterministic revisits.
Terrain and the resting camera remain stationary; idle updates do not run
the evaluator or rebuild meshes once the nearby window is complete.

The splash waits for nearby terrain, then offers Enter with sound or Enter
muted. A real click creates/resumes WebAudio. Short procedural crunches
follow grounded distance traveled. Airborne and idle motion do not create
walking sounds. Landing creates two prints and a lower thud scaled by impact
speed. Sound can be toggled afterward. Audio nodes disconnect when finished.

The latest 256 world-space boot prints are retained as terrain-draped decals.
Clear tracks erases them; Reset returns to the start and clears the trail.
Prints are visual marks rather than edits to terrain geometry or collision.
Footsteps and SnowWind are now X_eTaL libraries, and SnowAudio selects native
audio commands. The old Rust stride/aging decisions have been removed.
Their vertices are calculated once at each footfall, not rebuilt by sampling
the entire history. Older prints roll out of the bounded history.

Chunk generation uses a single persistent sliced evaluator on the main thread,
requesting at most one chunk at a time. Workers and floating-origin precision
remain future improvements. Float32 positions and Int32 chunk indices impose
numeric limits at extreme distances, despite the absence of a gameplay wall.
The world reducer is pure except for the existing mesh publication slot.
Rust owns streaming, navigation, decals, WebAudio and GPU integration.

## Preview and validation

The user prioritizes published feature feedback for this entertainment and
educational project: publish a buildable preview, then run the gate and fix
regressions in follow-up releases. Pending checks remain explicitly recorded.

Native scene tests cover stable rest, movement beyond the old boundary,
footprints, no airborne walking crunch and distinct jump/landing events.
Shader validators cover the shared camera, moving sun origin and print pass.
Run python3 scripts/check-demo.py --performance on a normal WebGPU host to
check real gesture-enabled audio, source views, idle stability, streamed
walking, prints, jump/landing, toggles, reset and four rendering modes.
AudioContext state checks do not establish audibility; listen in a visible
browser and inspect the screenshots. The agent sandbox denies local sockets.

The chunk_cost example records native release CPU loading, row-generation
and idle-scheduler timings. These exclude GPU, browser audio and physics
and do not measure FPS. Logical merged terrain storage is 614,400 bytes;
footprint vertices are capped at 30,720 bytes and the sun depth map at 1 MiB.

See [library APIs and source ownership](libraries.md).
