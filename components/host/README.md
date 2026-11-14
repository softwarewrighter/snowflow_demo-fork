# Host component

Typed host boundary, events, worker messages and resource lifecycle.

This component is an independent Cargo workspace containing `xetal-world-host`.
It owns validated meshes through bounded, instance-scoped handles with explicit
disposal and reset. See [mesh contract](../../docs/mesh-contract.md),
[architecture](../../docs/architecture.md) and [development](../../docs/development.md).
