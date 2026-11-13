# X_eTaL snowy world roadmap

Build an endless, editable snowy world in X_eTaL, evolving toward Snowflow's visual quality. Retain flowing water, torches, dirt, digging and gem hunting from the endless voxel demos. Replace trees with ice boulders and grass with snow. Terrain and water must have continuous visible surfaces; props must not be cubes. Snowflow is the visual reference, while the existing X_eTaL world supplies the gameplay reference.

This is a proposed architecture and staged implementation plan. No browser renderer or new language feature is implemented by this document. Source observations refer to the commits in `sources.lock.json`, copied from local cached origin/main refs; network freshness was not verified.

## Representation

Separate simulation, surface construction and rendering. A chunked sampled field is an implementation detail and does not require a block-shaped appearance. Use a signed density field for editable solid terrain, separate material channels for snow, dirt and rock, and separate water state. Define density as positive inside solids, with the zero crossing as the surface; outward normals follow the negative gradient. Use meters, seconds, right-handed xyz with y up, and linear RGB, consistent with the existing Scene contract.

A heightfield alone is insufficient for tunnels, overhangs and buried gems. Start with a deterministic height-based base density, then apply local volumetric edits. Store sparse edit records and generate chunks on demand. Begin with marching tetrahedra for a small reference mesher; evaluate marching cubes or dual contouring when triangle cost, ambiguity or feature preservation justifies it. Shared sample coordinates, halo samples and deterministic interpolation must prevent cracks across equal-resolution chunk boundaries. Defer mixed-resolution boundaries until transition geometry is implemented.

Digging applies a spherical brush to density rather than deleting one visible cube. Material classification must expose dirt below the snow layer. Ice boulders are continuous implicit shapes or generated meshes with matching collision geometry. Gems are separate faceted objects, revealed and collected through explicit gameplay rules. Torches are separate objects with local light and later flame effects.

Initially retain the existing cellular water model as a behavioral baseline, adapting its units and terrain occupancy tests. Construct connected surfaces from fill levels and add visible streams for falls; merely rounding each water cube is insufficient. Label existing replenishing sources explicitly. Do not claim fluid volume conservation: the current source/spread rules need separate finite-volume tests before that promise is made. Improve water simulation after terrain and digging work.

## Existing foundations

| Source | Reuse | Qualification |
| --- | --- | --- |
| X_eTaL xetal-play Interactive | Sliced execution, seeded runs, input and event waits | Existing implementation retains loaded program/output for a run; worker termination is part of lifecycle. Audit repeated resets for leaks. |
| X_eTaL browser runner | Worker execution and browser interaction | Extend its protocol; do not assume transferable mesh buffers or worker rendering already exist. |
| X_eTaL-libraries Vector3, Matrix, Geometry, Random | Math, transforms and deterministic generation | Confirm each operation's runtime cost on representative arrays. |
| X_eTaL-libraries Scene and Extrusion | Numeric scene contracts, construction and procedural meshes | The library Scene differs from the native extension also named Scene. Preserve existing contracts with an adapter. |
| X_eTaL-libraries RayTrace and Glass | Reference rendering and optical algorithms | Their existence does not establish real-time Snowflow performance. |
| X_eTaL-extensions Endless, Voxels and Play demos | Chunk generation, editing, player control, water, lights and Gem Hunt rules | Extract computation from native scene/window calls. Replace frame-count-based game time with fixed simulation ticks. |
| X_eTaL-extensions extension registry and static shell | Typed facade conventions and static linking | Public browser extension pages were retired; rebuild verification is required. |
| X_eTaL-gpu lowering and XIR | Source-to-array IR, maps, lookups, rotations, reductions, matrices and regions | Current runtimes are OpenCL/Mojo, not browser WebGPU. New backend coverage must be explicit. |
| X_eTaL-ML microscope | X_eTaL source and array inspection in Yew/WASM | Use as a UI reference, avoiding another copied execution engine. |

## Ownership of building blocks

Put domain algorithms in X_eTaL whenever the language can express them. Put platform access and execution mechanisms in Rust. An optimization may replace an implementation while keeping the typed contract and X_eTaL reference oracle. Do not silently move all simulation into Rust to meet a benchmark.

| Building block | X_eTaL library responsibility | Rust extension or compiler responsibility |
| --- | --- | --- |
| World and chunks | Seeded generation, coordinates, density, materials, edit journal, active chunks | Storage and bulk data transport; platform file/download access |
| Surface construction | Reference mesher, topology and normal rules | GPU execution of lowered kernels; optional accelerated implementation verified against reference |
| Input and movement | Movement policy, fixed-step integration, collision queries | Browser events, pointer lock, clocks and input queue |
| Digging and gems | Brushes, reveal/collect rules, inventory and game state | Ray-query acceleration and efficient buffer updates |
| Water | Reference flow rules, sources, fill state and stepping | GPU dispatch, persistent water buffers and synchronization |
| Lighting and materials | Light placement, snow coverage, material parameters and equations | Textures, samplers, pipeline state, render targets and submission |
| Scene | Named geometry, materials, transforms and pass descriptions | Validated persistent resources, draw submission and device lifecycle |
| Workers | Jobs, dependencies and cancellation policy exposed through typed APIs | Worker creation, message transfer, buffer ownership and scheduling |
| GPU compilation | Ordinary array programs using the supported subset | XIR-to-WGSL, scheduling, validation, capability checks and diagnostics |
| Assets | Procedural recipes and scene assembly | Optional Blender build tooling, glTF/image decoding and uploads |
| Inspection | Meaningful state and debug views | CPU/GPU timing, counters, capture and diagnostic transport |

Proposed library names such as WorldField, SurfaceMesh, WaterFlow and SnowWorld are placeholders, not current APIs. Extend existing Scene/Vector3/Random contracts where suitable. Keep rendering reusable across snow, water, terrain and unrelated applications.

## Language and runtime asks

| Ask | Placement | First proof |
| --- | --- | --- |
| Typed host registration with dense array arguments/results | X_eTaL host API, extending extensions ask E1 | One program uploads an array without formatting/parsing text; native and WASM results agree. |
| Persistent resources with ownership, bounds and disposal | Extension SDK initially; opaque resource types in core if needed | Stale resource use fails clearly; reset releases all scene resources. Integer IDs may bootstrap the interface. |
| Efficient indexing and bounded updates | Evaluator/compiler | Reproduce extensions ask E11; measure chunk extraction and edits without whole-world copying. |
| Resumable execution and event integration | Extend existing Interactive/step runtime | Continuous fixed-step world can pause, resume and reset without reparsing each frame. A transition budget is not a time bound on a long primitive. |
| Worker transfer and cancellation | Browser host plus typed facade | Obsolete chunk jobs cannot overwrite newer edits; bounded queues prevent growth while moving. |
| Device numeric policy | GPU schedule and diagnostics | Explicit f32/i32/u32 subset, overflow/range checks and tolerances; no silent loss of i64 IDs or f64 semantics. |
| GPU-resident arrays and interoperation | GPU runtime shared with renderer | Compute writes terrain data consumed by rendering without a CPU readback. |
| Shader authoring from language source | Compiler evolution | X_eTaL array kernels generate WGSL first; texture sampling, derivatives and graphics stage contracts are separately specified. |
| Diagnostics and profiling | Core spans plus extension diagnostics | Failure names the X_eTaL call, resource and backend; fallback execution is visible in performance reports. |

Cross-reference upstream asks before filing new ones. Each ask needs a minimal failing program, expected type/behavior, correctness oracle, benchmark where relevant, workaround, owner and removal condition. Workarounds must not change application semantics.

## Browser execution

Start with UI/input and rendering on the main thread, and generation/meshing in a dedicated WASM worker. Transfer owned message buffers and send small event batches, using bounded queues and versioned chunk requests. Main-thread uploads consume a bounded per-frame budget. Establish this before moving rendering to an OffscreenCanvas worker. Keep the device and its resources in one owner; do not assume GPU handles can be freely transferred between workers.

The base GitHub Pages build must work without shared memory. Shared-memory WASM threading is a later optional profile requiring cross-origin isolation and an appropriate hosting strategy. Worker messaging itself does not require shared memory. Generated JavaScript loader/worker glue is acceptable; application logic is Rust and X_eTaL, with no manually authored WASM. Start with adapted reference WGSL for graphics; full X_eTaL shader generation is a later deliverable.

Relevant browser contracts: [OffscreenCanvas](https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas), [shared memory security requirements](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer), and [crossOriginIsolated](https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated).

## Delivery stages

Each stage ends with a browser demo, documented contracts, a replay and measured evidence. These are acceptance gates, not completed features.

| Stage | Visible result | Acceptance evidence |
| --- | --- | --- |
| 0 | Tiny X_eTaL-controlled procedural mesh in browser | Typed bulk upload; persistent event loop; worker round trip; disposal/reset; Pages subdirectory build |
| 1 | Walkable smooth snowy chunks with ice boulders | Fixed seed reproduces fields; border samples and normals agree; bounded chunk memory while traveling |
| 2 | Dig through snow into dirt and reveal gems | Spherical edits cross chunk boundaries without cracks; collision follows edited surface; save/load reproduces edits |
| 3 | Water spills through a dug opening | Reference source/flow rules replay; no cube surfaces; terrain/water intersections and falling streams are checked |
| 4 | Torches, local lighting and gem hunting | Night scene, torch placement, deterministic collection, replayed fixed-step timer |
| 5 | Snowflow-inspired lighting and materials | Shadows, atmosphere, snow shading, ice, water and post-processing added individually with before/after captures and GPU costs |
| 6 | Larger simulation on browser GPU | Selected X_eTaL kernels lower through XIR; compare evaluator/XIR/WebGPU results; report unsupported operations explicitly |

GPU acceleration can move earlier when measurements justify it. Do not block the first playable world on a complete GPU compiler, realistic fluid solver, skeletal character, cloth, or all Snowflow spells.

## Reproducible process

Run `python3 scripts/inspect-xetal.py` to verify the inspected commit set and show dependency pins. It does not compile the ecosystem. Work copies are read-only research inputs by convention; changes belong in the owning repository and reach this application through reviewed commit updates.

1. Pin repository commits, Rust toolchain, Cargo.lock, bundler, browser version, resolution, seed and quality profile. Each sibling's XETAL_COMMIT remains authoritative for its own build; do not overwrite all pins with the latest core commit.
2. Make a minimal native evaluator oracle for each new algorithm. Record units, array shapes, indexing, winding, normal conventions and error cases beside the facade.
3. Compare the same small workload in evaluator, XIR interpreter and implemented GPU backends. Integer results must match exactly; floating comparisons declare absolute/relative tolerance and known nondeterminism.
4. Replay fixed input events at fixed simulation ticks. Hash generation and edit data; use tolerant image comparisons on declared reference hardware rather than promising identical GPU pixels everywhere.
5. Build WASM and test in a real WebGPU browser. Capture console errors, screenshots, CPU frame percentiles, GPU time when supported, active chunks, queue depth, upload bytes and memory estimates. Performance gates name their hardware and budget; correctness CI must not rely on an unspecified GPU.
6. Publish the static artifact to Pages with a capability report, source commit, seed, controls and quality settings. Include unsupported-device feedback. Validate worker and WASM paths under the repository URL.
7. Archive the manifest, commands, replay, results and images with each milestone. A clean-checkout run must reproduce the functional result without developer-local assets or sibling paths.

First implementation tasks are a browser build baseline and typed mesh-upload spike, then extraction of Endless/Play gameplay into host-independent libraries. Browser bindings, resource ownership and measured bridge costs decide subsequent work.
