#!/usr/bin/env python3
"""Prepare the built static demo for the gh-pages publishing branch."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def stage(source, destination, revision):
    files = list(source.iterdir())
    allowed = {'.html', '.js', '.wasm', '.css', '.ico', '.svg'}
    if not (source / 'index.html').is_file():
        raise ValueError('built index.html is missing')
    if any(p.is_symlink() or not p.is_file() or p.suffix not in allowed for p in files):
        raise ValueError('unexpected static artifact; refusing publication')
    destination.mkdir(parents=True, exist_ok=True)
    for path in files:
        shutil.copyfile(path, destination / path.name)
    for name in ['LICENSE', 'COPYRIGHT']:
        shutil.copyfile(ROOT / name, destination / name)
    (destination / '.nojekyll').touch()
    info = {'source_commit': revision, 'artifacts': {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in files}}
    (destination / 'build-info.json').write_text(json.dumps(info, indent=2) + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--destination', type=Path, default=ROOT / 'work/pages-publish')
    args = parser.parse_args()
    revision = subprocess.check_output(['git', '-C', str(ROOT), 'rev-parse', 'HEAD'], text=True).strip()
    stage(ROOT / 'work/demo-site/snowflow_demo-fork', args.destination, revision)
    print(f'Prepared Pages artifact at {args.destination}; source {revision}')


if __name__ == '__main__':
    main()
