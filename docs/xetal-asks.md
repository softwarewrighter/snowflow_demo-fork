# Host binding requirements

The application uses existing X_eTaL syntax and numeric conventions. This file
tracks its integration needs; it does not implement language semantics or claim
an upstream request has landed.

## Typed host registration

Status: existing extensions ask E1 is filed upstream; this application adds a
concrete workload and acceptance criteria rather than filing a duplicate.
Owner: X_eTaL's evaluator/type checker, with shared ABI work in X_eTaL-extensions.

The required workload is an N by 3 Float coordinate array and a T by 3 Int
vertex-index array, retained as one application mesh resource. Existing Scene
T by 3 by 3 triangles must also be accepted through an explicit adapter.
Malformed shapes, nonfinite coordinates and invalid indices must return a
diagnostic identifying the caller. Updates, disposal and reset must release
resources without letting an old token name a later mesh.

The existing ext: workaround formats dense numeric arrays as text, then parses
them back. The first app instead uses the existing output hook to retain the
program's lexical environment and read its typed frame (mesh, print and sound commands) after event waits.
Only the publication slot frame! remains mutable: logical state is an immutable
tuple passed through pure reducers and geometry functions. A typed publication
operation callable inside the event loop would remove this last slot. The
slot is a host API workaround, not a measured performance requirement.
This application-specific adapter copies/narrows typed values without formatting
them; it is not a generic registered native call. The native mesh_transfer example
provides a repeatable serialization workload and checked conversion reference;
it is not an ext: benchmark or a reproduction of a missing built-in.

Integration acceptance:

1. Register typed array arguments/results with the host and type checker.
2. Route dense values through the existing ABI without Float-encoding IDs.
3. Explicitly convert f64 coordinates to f32 under the application policy.
4. Keep reusable ownership/transport in the SDK; application geometry policy
   stays in surface. Compare typed and ext: execution of the same X_eTaL program.
5. Report caller spans and resource errors; verify native and actual browser
   execution, resource reset and bounded transfers.

The workaround is the application contract, captured mesh binding and native tests.
Replace the representation-dependent adapter when a reviewed upstream typed binding is pinned
and the same workload runs from X_eTaL without text conversion. No new core
resource type, syntax or effect rule is silently decided by this prototype.

Reference: [existing extensions asks](https://github.com/softwarewrighter/X_eTaL-extensions/blob/main/docs/xetal-asks.md).

## Measured editable-terrain workload

Sweep adds a concrete performance workload for worker execution, typed bulk
arrays and faster array kernels. The current native release Scene update takes
about 16 ms for one cut and 39 ms with eight retained cuts, excluding GPU/audio
and footprints. See [measurements and reproduction](evidence/sweep.json).
Application policy stays in the pure Sweep/TerrainBrush libraries; moving those
rules into native application code would hide the language workload.

Dirty chunks are committed atomically so collision and rendering agree. The
current few-scalar Event transport and typed output capture do not provide a
generic way to pass cached bulk arrays back into the evaluator. E1 would help
replace this bridge and expose reusable cache/array capabilities. Worker jobs
must preserve edit revisions and atomic publication, coalesce obsolete work,
and bound queues and memory. This is local evidence for existing requirements,
not a claim that a new upstream API or GPU compiler has been implemented.
