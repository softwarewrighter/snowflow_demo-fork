# Render component

GPU resource and rendering integration.

This component is an independent Cargo workspace containing `xetal-world-render`.
It uploads validated meshes through wgpu, reuses buffers for matching sizes,
handles resize and recoverable surface errors, and draws a depth-tested snow
surface. The browser code is WASM-only. A native Naga test validates the shader.
See [local demo](../../docs/local-demo.md) and [architecture](../../docs/architecture.md).
