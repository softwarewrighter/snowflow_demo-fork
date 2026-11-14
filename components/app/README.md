# App component

Browser application composition and inspection.

This component is an independent Cargo workspace containing `xetal-snow-app`.
It composes a persistent X_eTaL evaluator, a typed snapshot adapter, WebGPU
rendering and browser controls. Static page sources are under `web/`.
See [local demo](../../docs/local-demo.md), [architecture](../../docs/architecture.md)
and [development](../../docs/development.md).

The workspace also contains native-testable navigation, bounded footprint
recording and procedural sound synthesis. Browser WebAudio is gesture-created
by the start screen. Scene coordination has native tests for idle stability,
streaming beyond the old arena, footprints and jump/landing events.
