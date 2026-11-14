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
them back. The application contract accepts typed arrays without that round
trip, but no evaluator hook connects to it yet. The native mesh_transfer example
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

The workaround is the Rust-side application contract and direct native tests.
Remove that integration gap when a reviewed upstream typed binding is pinned
and the same workload runs from X_eTaL without text conversion. No new core
resource type, syntax or effect rule is silently decided by this prototype.

Reference: [existing extensions asks](https://github.com/softwarewrighter/X_eTaL-extensions/blob/main/docs/xetal-asks.md).
