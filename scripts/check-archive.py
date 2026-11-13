#!/usr/bin/env python3
"""Check every preserved reference file against its committed source manifest."""
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]


def failures(manifest):
    archive = ROOT / manifest["root"]
    expected = {entry["path"]: entry["sha256"] for entry in manifest["files"]}
    actual = {str(path.relative_to(archive)) for path in archive.rglob("*") if path.is_file()}
    for name in sorted(set(expected) - actual):
        yield f"missing: {name}"
    for name in sorted(actual - set(expected)):
        yield f"unexpected: {name}"
    for name in sorted(actual & set(expected)):
        if hashlib.sha256((archive / name).read_bytes()).hexdigest() != expected[name]:
            yield f"changed: {name}"
    if (ROOT / "orig-README.md").read_bytes() != (archive / "README.md").read_bytes():
        yield "orig-README.md differs from the original README"


def main():
    manifest = json.loads((ROOT / "docs/archive.lock.json").read_text())
    errors = list(failures(manifest))
    for error in errors:
        print(f"archive: {error}", file=sys.stderr)
    if not errors:
        print(f"archive: {len(manifest['files'])} files match {manifest['commit']}")
    return bool(errors)


if __name__ == "__main__":
    sys.exit(main())
