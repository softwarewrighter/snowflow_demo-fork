# Application design decisions

This register records application decisions. Language semantics remain governed
by X_eTaL's `docs/lang-choices.md` and executable specification. Proposed language
features require an upstream ask and test-first implementation in their owner.

| ID | Decision | Reason |
| --- | --- | --- |
| D1 | Smooth snowy endless world retains digging, water, torches, dirt and gems | User-selected application and gameplay scope |
| D2 | Replace trees with ice boulders and grass with snow | User-selected visual direction |
| D3 | Chunked density representation with continuous visible surfaces | Supports streaming, digging, tunnels and overhangs without cube-shaped terrain |
| D4 | X_eTaL algorithms with Rust platform extensions | Dogfood the language and expose reusable missing capabilities |
| D5 | Independent component workspaces with a shared target directory | Follow the X_eTaL repository architecture |
| D6 | Preserve original source and attribution under archive | Retain a reproducible reference and its original license |
| D7 | Generated browser glue is acceptable; no handwritten application JS or WASM | Keep application development in X_eTaL and Rust |
| D8 | Base browser profile uses message workers without shared memory | Keep the first Pages build independent of cross-origin isolation |
| D9 | Evolve from small visible milestones toward Snowflow quality | Establish correctness and budgets before advanced rendering |
| D10 | Typed owned f32/u32 payloads with checked f64/i64 adapters | Keep serialization outside the Rust upload boundary and make narrowing explicit |
| D11 | Instance-scoped mesh handles never wrap or alias disposed resources | Preserve ownership through disposal/reset; keep scene IDs separate |

The native mesh contract can be developed independently of browser execution.
Browser-baseline validation remains mandatory before the browser mesh demo;
its unavailable environment must not prevent native contract work.

## Upstream feature requests

Use [the implementation plan](plan.md) to schedule requests. Existing extensions
ask E1 covers typed host bindings; E11 records costly scalar selection. Reproduce
these against the selected core pin before creating duplicate requests. Add
resource lifetime, bulk data transfer, GPU residency and worker cancellation
requests with minimal programs, type contracts, correctness oracles, benchmarks
and explicit workaround removal conditions.

XIR already supports maps, selection, lookup, rotations, reductions, matrices and
regions. WebGPU execution is new backend work, not evidence that those operations
are absent from the language. New texture/graphics operations need separate
contracts and decisions. Do not silently narrow i64/f64 semantics for a GPU.
