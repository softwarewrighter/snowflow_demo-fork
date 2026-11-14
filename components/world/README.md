# World component

World fields, materials, chunks and edit data contracts.

This component is an independent Cargo workspace containing `xetal-world-model`.
It runs `demos/snowfield.xtl` through the pinned sliced evaluator and reads its
mesh binding as typed data. Native tests cover geometry, events, reset, separate
program instances and input errors. Reusable world libraries remain separate
from this initial application program. See [local demo](../../docs/local-demo.md).

The world reducer and geometry functions use immutable tuples. A single mutable
mesh publication slot connects the effect loop to the current Rust host API.
Tests include legacy/new replay equivalence, upstream X_eTaL linting and an AST
guard for that boundary. The compare_state example measures the legacy and pure
variants with identical traces; see [performance](../../docs/functional-state.md).
