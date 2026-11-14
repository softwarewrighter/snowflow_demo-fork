# World component

World fields, materials, chunks and edit data contracts.

This component is an independent Cargo workspace containing `xetal-world-model`.
It runs `demos/snowfield.xtl` through the pinned sliced evaluator and reads its
mesh binding as typed data. Native tests cover geometry, events, reset, separate
program instances and input errors. Reusable world libraries remain separate
from this initial application program. See [local demo](../../docs/local-demo.md).
