# Application architecture

X_eTaL owns procedural world algorithms and gameplay. Rust owns platform access
and execution mechanisms. The application composes those reusable facilities.
Host and surface implement mesh payloads and resource lifetime. World runs a
persistent X_eTaL surface program; render supplies WebGPU and app supplies the
browser composition. Shared replay/evidence code remains scaffolding.

```text
X_eTaL generation and gameplay libraries
                  |
        typed host and array contracts
                  |
       world -> surface -> render
                  |
            browser app
                  |
          replay and evidence
```

`world` and `surface` data must remain usable without a DOM or GPU. `host`
defines event, resource and worker boundaries without importing the app.
`render` consumes validated scene and mesh data; gameplay cannot depend on its
pipeline internals. `app` wires input, evaluator, jobs and rendering together.
`evidence` observes typed events and results without changing simulation rules.

Use the existing Scene numeric contracts through explicit adapters. The Scene
library and native Scene extension are different facilities; neither is already
a WebGPU scene renderer. Preserve deterministic reference algorithms in X_eTaL
while adding accelerated implementations behind the same contracts.

Keep chunks as a simulation and streaming structure. Their visible solid surface
comes from a continuous density field, so digging can create tunnels and
overhangs. Water state is separate from terrain density and from the visible
water mesh. Snow coverage, dirt and rock are material channels. Ice boulders,
gems and torches are objects with suitable geometry and collision rules.

Start with a main-thread GPU owner and a worker for generation and meshing.
Use bounded queues, transferred buffers and versioned chunk requests. Reuse the
existing sliced evaluator, event waits and browser worker foundations. Shared
memory and worker-owned rendering are optional later profiles, each requiring
real browser capability checks.

The initial demo runs the evaluator and renderer on the main thread. Its typed
adapter uses the existing output hook to retain a closure's lexical environment,
then reads the updated mesh binding after event waits. This requires no core
syntax change but depends on the pinned public runtime representation. Generic
host registration remains an upstream feature request. See [local demo](local-demo.md).

The effects component adds the Bloom native adapter and a reusable instanced
particle renderer. Spell and particle algorithms remain in project-local X_eTaL
libraries. World streaming consumes separate pending Sweep and Bloom commands,
then publishes one shared edited mesh to rendering and collision. The raycast
backend reports actual triangle hits; targeting limits come from the Bloom
library. See [Bloom](bloom.md) for current APIs, budgets and acceptance limits.
