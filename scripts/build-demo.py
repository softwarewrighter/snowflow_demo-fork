#!/usr/bin/env python3
"""Build the Rust/WASM snow-surface demo into work/demo-site/."""
import os
from pathlib import Path
import subprocess
import json
from cargo_cache import prepare

ROOT = Path(__file__).resolve().parents[1]


def environment():
    env = dict(os.environ, NO_COLOR='true', CARGO_NET_OFFLINE='true')
    source = Path(env.get('CARGO_HOME', str(Path.home() / '.cargo')))
    env['CARGO_HOME'] = str(prepare(source, ROOT / 'work/cargo-home'))
    expected = json.loads((ROOT / 'docs/tools.lock.json').read_text())['wasm-bindgen']
    candidate = Path.home() / '.cache/trunk' / expected.replace(' ', '-')
    if (candidate / 'wasm-bindgen').exists():
        env['PATH'] = str(candidate) + os.pathsep + env['PATH']
    return env


def main():
    if not (ROOT / 'work/deps/X_eTaL').exists():
        raise RuntimeError('Run python3 scripts/bootstrap-xetal.py before building the demo')
    env = environment()
    target = ROOT / 'work/demo-site/snowflow_demo-fork'
    command = ['trunk', 'build', '--offline', '--locked', '--public-url', '/snowflow_demo-fork/', '--dist', str(target)]
    subprocess.run(command, cwd=ROOT / 'components/app/web', env=env, check=True)
    print(f'Demo built: {target}')


if __name__ == '__main__':
    main()
