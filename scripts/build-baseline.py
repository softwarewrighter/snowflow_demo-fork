#!/usr/bin/env python3
"""Build the pinned upstream evaluator and worker without editing its sources."""
import json
import hashlib
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def record_evidence(dist, revision, tools):
    files = {}
    for path in sorted(dist.glob('*.wasm')):
        files[path.name] = {'bytes': path.stat().st_size, 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}
    if len(files) != 2:
        raise RuntimeError(f'expected editor and worker WASM, got {list(files)}')
    evidence = {'core_commit': revision, 'tools': tools, 'native_suites': 'xetal-play and xetal-runner passed', 'wasm_build': 'passed', 'browser_execution': 'not run by build script', 'artifacts': files}
    (ROOT / 'work/baseline-evidence.json').write_text(json.dumps(evidence, indent=2) + '\n')


def main():
    core = ROOT / 'work/deps/X_eTaL'
    lock = json.loads((ROOT / 'docs/sources.lock.json').read_text())
    expected = next(s['commit'] for s in lock['sources'] if s['name'] == 'X_eTaL')
    actual = subprocess.check_output(['git', '-C', str(core), 'rev-parse', 'HEAD'], text=True).strip()
    if actual != expected:
        raise RuntimeError('core checkout does not match sources.lock.json')
    if subprocess.check_output(['git', '-C', str(core), 'status', '--porcelain'], text=True).strip():
        raise RuntimeError('core checkout has local changes; refusing baseline build')
    tools = json.loads((ROOT / 'docs/tools.lock.json').read_text())
    for name in ['rustc', 'cargo', 'trunk', 'wasm-bindgen']:
        version = subprocess.check_output([name, '--version'], text=True).strip()
        if version != tools[name]:
            raise RuntimeError(f'{name}: expected {tools[name]}, found {version}; select matching tool on PATH')
    subprocess.run(['cargo', 'test', '--offline', '--locked', '--manifest-path', str(core / 'components/web/Cargo.toml'), '-p', 'xetal-play', '-p', 'xetal-runner'], check=True)
    env = dict(os.environ, NO_COLOR='true')
    dist = ROOT / 'work/baseline-site/snowflow_demo-fork'
    subprocess.run(['trunk', 'build', '--offline', '--locked', '--public-url', '/snowflow_demo-fork/', '--dist', str(dist)], cwd=core / 'components/web/crates/xetal-web', env=env, check=True)
    record_evidence(dist, expected, tools)
    print(f'Built upstream evaluator baseline at {dist}; not the snowy world app.')


if __name__ == '__main__':
    main()
