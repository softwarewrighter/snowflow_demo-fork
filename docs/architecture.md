# Application architecture

X_eTaL owns procedural world algorithms and gameplay. Rust owns platform access
and execution mechanisms. The application composes those reusable facilities.
The component facades are scaffolding; the following dependency boundaries guide
implementation and do not claim an existing renderer.

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
