#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

export CARGO_HOME="$(python3 - <<'PY'
import os
from pathlib import Path
import sys
sys.path.insert(0, 'scripts')
from cargo_cache import prepare
print(prepare(Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo'))), Path('work/cargo-home')))
PY
)"

git diff --check
python3 scripts/check-archive.py
python3 scripts/check-spelling.py --self-test
python3 scripts/check-spelling.py
python3 -m unittest discover -s tests -v
sw-markdown-checker -f README.md
python3 scripts/check-instructions.py
sw-markdown-checker -p docs -f '*.md'
sw-markdown-checker -p components -f '**/*.md'
sw-markdown-checker -p archive -f README.md

for component in components/*/Cargo.toml; do
    cargo-fmt fmt --manifest-path "$component" --all
    cargo-fmt fmt --manifest-path "$component" --all -- --check
    cargo-clippy clippy --offline --manifest-path "$component" --all-targets --all-features -- -D warnings
    cargo test --offline --manifest-path "$component"
    sw-checklist "$(dirname "$component")"
done

cargo-clippy clippy --offline --manifest-path components/app/Cargo.toml --target wasm32-unknown-unknown --all-targets -- -D warnings
