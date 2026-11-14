# Libraries used by the snowfield

The live program is [snowfield.xtl](../components/world/demos/snowfield.xtl).
It imports actual project-local X_eTaL modules. Their public functions and
rendered source are generated into the demo's library-reference.html page;
rendered calls link directly to their module. These are app modules, not
claims that new packages have already shipped in X_eTaL-libraries.

| Module | Alias | Public API and responsibility | Source |
| --- | --- | --- | --- |
| SnowTerrain | snow: | h_eight and m_esh; deterministic world-space terrain arrays | [SnowTerrain.xtl](../components/world/libs/SnowTerrain.xtl) |
| Footsteps | feet: | s_tart, u_pdate, t_ick, v_iew, c_ount, l_ands, w_ind; stride, alternating soles, airborne suppression, landings and bounded history | [Footsteps.xtl](../components/world/libs/Footsteps.xtl) |
| SnowAudio | sfx: | c_ommand and s_ilent; walking versus landing buffer, gain and playback rate | [SnowAudio.xtl](../components/world/libs/SnowAudio.xtl) |
| SnowWind | wind: inside Footsteps | s_trength and o_pacity; timed gusts and age/exposure fill-in | [SnowWind.xtl](../components/world/libs/SnowWind.xtl) |
| SnowHost | host: | n_ext!, c_hunk and p_ose; facade for queued native facts and requests | [SnowHost.xtl](../components/world/libs/SnowHost.xtl) |

Footsteps and SnowAudio now own the decisions previously implemented in
Rust Trail::update and Scene::motion. Rust no longer decides stride timing,
which foot to stamp, when a semantic snow sound occurs or its artistic gain.
The world and player evaluator instances use the same library-led program:
one receives chunk requests; the other receives pose and time updates.

## Host contract and native backends

The immutable model emits one typed frame:
(mesh, print rows, audio command, step count, landing count, wind strength).
Only frame! is mutable, to publish that value through the current evaluator
hook. Mesh and command arrays are copied/narrowed under the host's checks;
there is no dense numeric text serialization.

Print rows are [x, z, yaw, side, birth exposure, opacity]. The print backend
caches terrain-draped vertices and uses the supplied opacity. Audio commands
are [buffer, gain, rate]: 0 means no new sound, 1 crunch and 2 thud. The audio
backend executes that command and releases finished nodes. It does not decide
when a player should step or land.

| Native capability | Implementation | Role |
| --- | --- | --- |
| X_eTaL evaluation and checking | [runtime adapter](../components/world/crates/xetal-world-model/src/runtime.rs) | Pinned X_eTaL evaluator, macro loader and type checker |
| Embedded library lookup | [catalog](../components/world/crates/xetal-snow-libraries/src/lib.rs) | Real library sources available in native and WASM builds |
| Pose transport | [player adapter](../components/world/crates/xetal-world-model/src/player.rs) | Encodes a few native facts into queued Events, coalesces poses and transports typed output |
| Navigation physics | [navigation](../components/app/crates/xetal-world-navigation/src/lib.rs), [pure motion](../components/app/crates/xetal-world-navigation/src/physics.rs) | Native position, yaw, grounded flag and impact speed |
| GPU print cache | [trail backend](../components/app/crates/xetal-world-trail/src/recorder.rs) | Geometry caching; no stride or aging policy |
| Audio execution | [WebAudio](../components/app/crates/xetal-world-audio/src/browser.rs), [buffer/node ownership](../components/app/crates/xetal-world-audio/src/bank.rs), [DSP](../components/app/crates/xetal-world-audio/src/synth.rs) | Gesture-enabled audio, procedural waveform buffers and node cleanup |
| Drawing and depth maps | [render](../components/render/crates/xetal-world-render/src/lib.rs), [decals](../components/render/crates/xetal-world-decals/src/lib.rs), [shadows](../components/render/crates/xetal-world-shadow/src/lib.rs) | WebGPU resource ownership and shaders |

SnowHost currently uses the existing Event transport: choose carries chunk
coordinates, move carries player x/z, down/up carries grounded/airborne yaw
and impact, and tick carries elapsed time. These are documented adapter
messages, not a newly registered native-function API. The generic typed
host-binding feature E1 remains an [upstream requirement](xetal-asks.md).
Physics and camera control remain native wrappers; this release moves the
footstep, sound-command and snow-aging policies into X_eTaL.

## Wind and aging

SnowWind supplies a calm background and a gust every 12 seconds. Each mark
records wind exposure at birth. Opacity decreases with accumulated exposure,
so older marks disappear first and gusts accelerate the fill-in. The default
lifetime is 25 exposure units, approximately 40 seconds over a full gust cycle.
The history is capped at 256 marks. Pause freezes simulation time.

## Build and verify

Run python3 scripts/build-demo.py. It generates the live reference from the
same embedded source catalog and the build's commit, including public-function
line links. No reference entry is generated for an unimplemented library.

Native tests include [player library behavior](../components/world/crates/xetal-world-model/tests/player_libraries.rs),
[whole scene](../components/app/crates/xetal-snow-app/tests/scene.rs),
[mutation and lint checks](../components/world/crates/xetal-world-model/tests/purity.rs)
and [geometry/opacity caching](../components/app/crates/xetal-world-trail/tests/tracks.rs).
The browser harness remains the place for actual audible/visual/FPS evidence.

The previously queued overcast sky, faint sun, night/moon and carried torch
remain a later phase; library ownership and references were prioritized at
the user's request.

Core primitives such as reshape, select, transpose, arithmetic and Event
access come from the pinned X_eTaL runtime, not additional app libraries.
Their declarations are in the [pinned builtin catalog](https://github.com/softwarewrighter/X_eTaL/blob/c5750cec6af7063a90818046c7c7945db71b4a36/components/base/crates/xetal-catalog/builtins.toml).
Source pins are recorded in [sources.lock.json](sources.lock.json).
