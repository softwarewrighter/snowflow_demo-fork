#!/usr/bin/env python3
"""Verify research snapshots and print upstream dependency pins; never mutate them."""
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def git(path, *args):
    return subprocess.check_output(
        ["git", "-C", str(path), *args], text=True, stderr=subprocess.PIPE
    ).strip()


def main():
    lock = json.loads((ROOT / "docs/sources.lock.json").read_text())
    failures = []
    for source in lock["sources"]:
        path = ROOT / source["path"]
        try:
            actual = git(path, "rev-parse", "HEAD")
            dirty = git(path, "status", "--porcelain", "--untracked-files=normal")
            if actual != source["commit"]:
                failures.append(f"{source['name']}: expected {source['commit']}, got {actual}")
            if dirty:
                failures.append(f"{source['name']}: snapshot has local changes")
            pin_file = path / "XETAL_COMMIT"
            pin = pin_file.read_text().strip() if pin_file.exists() else "self"
            if pin_file.exists() and pin != source.get("core_commit"):
                failures.append(f"{source['name']}: core pin differs from manifest")
            print(f"{source['name']} {actual} core-pin={pin}")
        except (OSError, subprocess.CalledProcessError) as error:
            failures.append(f"{source['name']}: {error}")
    for failure in failures:
        print(f"ERROR: {failure}", file=sys.stderr)
    print("Source inspection only; compilation and browser execution are separate gates.")
    return bool(failures)


if __name__ == "__main__":
    sys.exit(main())
