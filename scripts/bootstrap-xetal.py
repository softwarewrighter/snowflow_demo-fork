#!/usr/bin/env python3
"""Create exact dependency checkouts without writing to research snapshots."""
import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def git(path, *args):
    return subprocess.check_output(
        ['git', '-C', str(path), *args], text=True, stderr=subprocess.PIPE
    ).strip()


def checkout(source, revision, destination, offline=False):
    if len(revision) != 40 or any(c not in '0123456789abcdef' for c in revision):
        raise ValueError('a full lowercase commit hash is required')
    if not destination.exists():
        destination.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(['git', 'clone', '--no-hardlinks', source, str(destination)], check=True)
    if git(destination, 'status', '--porcelain', '--untracked-files=normal'):
        raise RuntimeError(f'{destination}: local changes; refusing checkout')
    found = subprocess.run(['git', '-C', str(destination), 'cat-file', '-e', revision + '^{commit}'], capture_output=True)
    if found.returncode:
        if offline:
            raise RuntimeError(f'{destination}: commit unavailable offline: {revision}')
        git(destination, 'fetch', 'origin', revision)
    git(destination, 'checkout', '--detach', revision)
    if git(destination, 'rev-parse', 'HEAD') != revision:
        raise RuntimeError(f'{destination}: checkout verification failed')


def bootstrap(args):
    lock = json.loads((ROOT / 'docs/sources.lock.json').read_text())
    core = next(s for s in lock['sources'] if s['name'] == 'X_eTaL')
    core_source = str(args.cache_dir / core['name']) if args.cache_dir else core['url']
    for source in lock['sources']:
        origin = str(args.cache_dir / source['name']) if args.cache_dir else source['url']
        destination = args.destination / source['name']
        checkout(origin, source['commit'], destination, args.offline)
        pin_file = destination / 'XETAL_COMMIT'
        if pin_file.exists():
            pin = pin_file.read_text().strip()
            if pin != source['core_commit']:
                raise RuntimeError(f"{source['name']}: core pin differs from manifest")
            checkout(core_source, pin, destination / 'work/xetal', args.offline)
        print(f"verified {source['name']} {source['commit']}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--destination', type=Path, default=ROOT / 'work/deps')
    parser.add_argument('--cache-dir', type=Path, help='read-only local Git repositories instead of network')
    parser.add_argument('--offline', action='store_true')
    args = parser.parse_args()
    if args.offline and not args.cache_dir:
        parser.error('--offline requires --cache-dir')
    args.destination = args.destination.resolve()
    if args.cache_dir:
        args.cache_dir = args.cache_dir.resolve()
    bootstrap(args)


if __name__ == '__main__':
    main()
