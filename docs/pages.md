# GitHub Pages preview

The preview URL is https://softwarewrighter.github.io/snowflow_demo-fork/.
GitHub supplies HTTPS; the browser needs no development certificate or
HTTP-origin override. Device/browser WebGPU support is still required.

The gh-pages branch contains the static build. Source remains on main. No local
server is required to open the hosted preview. The build-info.json file records
the source commit and artifact hashes. A successful HTTP deployment does not
replace browser rendering and interaction acceptance.

## Publish an update

Build with `python3 scripts/build-demo.py` (an optimized release build), and
commit the source changes before preparing the artifact. Per the preview policy,
publish the buildable preview first, then run the gate and publish any fixes:

```sh
python3 scripts/package-pages.py
```

The prepared directory is `work/pages-publish`. On first use initialize it as a
Git repository with branch gh-pages and set origin to this repository's HTTPS
URL. On later runs fetch and fast-forward that branch before preparing a new
artifact. Commit its generated files and push gh-pages without force. Push main
as well so the build identity resolves to published source.

The packaging allowlist accepts HTML, CSS, generated JS, WASM and the SVG snowflake favicon.
It adds the project notices and .nojekyll. Unexpected files and symlinks fail
before copying; private keys under work/tls are never a packaging input.

Pages settings use branch gh-pages and path /. HTTPS is enabled. GitHub's own
Pages build/deployment runs after that branch changes. Check deployment status
and verify the live index, build-info.json, hashed assets and application/wasm
MIME type before reporting publication complete.

No custom workflow OAuth permission is required for this branch-based path.
An automatic Rust build workflow can be added separately with the appropriate
repository workflow permissions. The local build and exact Cargo/source pins
remain the build procedure for this preview.
