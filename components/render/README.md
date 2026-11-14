# Render component

GPU resource and rendering integration.

This component is an independent Cargo workspace containing `xetal-world-render`.
It uploads validated meshes through wgpu, reuses buffers for matching sizes,
handles resize and recoverable surface errors, and draws a depth-tested snow
surface. The browser code is WASM-only. A native Naga test validates the shader.
See [local demo](../../docs/local-demo.md) and [architecture](../../docs/architecture.md).

The xetal-world-shadow crate owns a cached 512 by 512 directional depth map.
Its shared light projection is compiled into the depth and visible shaders.
The visible pass adds procedural material detail and comparison-filtered
shadows, independently disabled by application controls. See
[effects and costs](../../docs/lightweight-effects.md).

The xetal-world-camera crate shares the WGSL camera ABI. The small
xetal-world-decals crate draws bounded procedural boot prints in one extra
draw, using stored terrain-draped vertices. Sun depth origin follows the player
without changing the fixed light direction.
