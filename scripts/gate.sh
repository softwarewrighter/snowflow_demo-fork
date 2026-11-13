#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

git diff --check
python3 scripts/check-archive.py
python3 scripts/check-spelling.py --self-test
python3 scripts/check-spelling.py
sw-markdown-checker -f README.md
python3 scripts/check-instructions.py
sw-markdown-checker -p docs -f '*.md'
sw-markdown-checker -p components -f '**/*.md'
sw-markdown-checker -p archive -f README.md

for component in components/*/Cargo.toml; do
    cargo fmt --manifest-path "$component" --all
    cargo fmt --manifest-path "$component" --all -- --check
    cargo clippy --offline --manifest-path "$component" --all-targets --all-features -- -D warnings
    cargo test --offline --manifest-path "$component"
    sw-checklist "$(dirname "$component")"
done
