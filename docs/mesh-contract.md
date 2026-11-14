# Mesh arrays and resource lifetime

The surface component validates the application's mesh payload. The host
component retains those payloads with count and logical-upload-byte limits.
These are native and WASM-compatible Rust APIs, not a renderer, GPU upload,
X_eTaL facade or new language binding.

## Arrays and coordinates

| Input | Shape | Meaning |
| --- | --- | --- |
| Mesh::from_arrays positions | N by 3 f32 | Owned row-major xyz buffer, finite values |
| Mesh::from_arrays indices | T by 3 u32 | Owned zero-based vertex indices |
| from_scene triangles | T by 3 by 3 f64 | Existing Scene triangle order and vertex winding |
| from_xetal positions | N by 3 f64 | X_eTaL Float coordinates |
| from_xetal indices | T by 3 i64 | Application indexed convenience form, one-based vertex references |

Coordinates follow Scene: right-handed xyz, Y up, meters, radians and linear
color when colors are added. The existing Scene contract is unchanged.
Material IDs, group IDs and primitive IDs are not vertex indices or resource
handles. The Scene adapter expands each triangle into three vertices with
sequential indices; it does not weld geometry or reinterpret semantic IDs.

Shape products are checked for overflow and must match the buffers exactly.
Invalid rank, dimensions, nonfinite coordinates and out-of-range indices fail.
Empty meshes are valid. Degenerate triangles and winding are retained; a future
mesher must generate appropriate geometry rather than rely on silent repair.

The f64 adapters explicitly round to f32. They reject nonfinite input, values
outside finite f32 range and nonzero values that round to zero. Ordinary f32
rounding and nonzero subnormals are accepted; GPU treatment of subnormals still
needs backend validation. One-based i64 indices are checked before subtraction
and conversion to u32, so negative or very large IDs cannot wrap.

## Ownership and lifetime

The owned f32/u32 constructor retains the original Vec allocations. Borrowed
f64/i64 arrays require conversion and allocation. Neither path claims zero-copy
from the evaluator, across the ABI, into a worker, or onto a GPU.

MeshStore consumes a validated mesh. A handle belongs to the store that retained
it, within the current Rust execution instance. Lookups from another store fail.
Disposal drops the payload; repeated disposal fails. Reset drops all live meshes.
Monotonic handles are never reused, including after reset, and exhaustion fails
instead of wrapping. The registry uses no unsafe code.

Separate WASM instances have separate handle namespaces. Keep handles with their
owning renderer/runtime; do not persist raw values or route them between workers.
A future external protocol must include owner/session identity. A raw handle is
not authorization to recreate a Rust handle or access another instance.

Budget checks happen before insertion and leave live resources/accounting
unchanged on failure. Empty meshes consume count budget. Byte accounting is
positions plus indices at four bytes per item. It excludes spare Vec capacity,
HashMap overhead, GPU alignment and staging memory; it is not a memory profiler.

## Validation and measurement

The integration tests check allocation retention, Scene order, malformed shapes,
index conversion, precision failures, budgets, disposal, reset and cross-store
handle rejection. The host and surface libraries also compile for
wasm32-unknown-unknown. Compilation is separate from browser execution.

Run the native payload microbenchmark with:

```sh
cargo run --offline --release --manifest-path components/surface/Cargo.toml --example mesh_transfer
```

It measures 32 batches of 4096 triangles. The owned-f32 case prepares buffers
outside the timer, then validates and consumes them. The checked-f64 case times
conversion and validation. The text case additionally formats/parses coordinate
and index values before that same checked conversion. All cases consume the
result with black_box. Timings include payload destruction inside each loop.

This measures native payload preparation, not the actual ext: bridge, evaluator,
worker messaging, GPU transfer or frame time. It demonstrates a reason to keep
text serialization off the mesh path, without claiming a language speedup.
Observed timings and environment are recorded in the implementation evidence.

## Extension integration

The current APIs define application payload policy. Reusable ABI support and
typed evaluator hooks belong in X_eTaL-extensions and X_eTaL, respectively;
they are not duplicated here. Existing upstream ask E1 already requests typed
host registration. See [host binding requirements](xetal-asks.md) for the
integration contract, workaround and promotion conditions.
