#!/usr/bin/env python3
"""Build and serve the X_eTaL snow demo on the LAN; run from a normal terminal."""
import argparse
import functools
import http.server
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def options(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=8475)
    parser.add_argument('--bind', default='0.0.0.0', help='listening address (default: all interfaces)')
    parser.add_argument('--no-build', action='store_true')
    return parser.parse_args(argv)


def network_host(binding):
    if binding != '0.0.0.0':
        return binding
    try:
        rows = json.loads(subprocess.check_output(['ip', '-j', '-4', 'addr', 'show', 'scope', 'global'], text=True))
        addresses = [a['local'] for row in rows for a in row.get('addr_info', []) if a.get('family') == 'inet']
        return next((a for a in addresses if a.startswith('192.168.')), next(iter(addresses), '127.0.0.1'))
    except (OSError, subprocess.CalledProcessError, json.JSONDecodeError):
        return '127.0.0.1'


def main():
    args = options()
    if not args.no_build:
        subprocess.run([sys.executable, str(ROOT / 'scripts/build-demo.py')], check=True)
    site = ROOT / 'work/demo-site'
    if not (site / 'snowflow_demo-fork/index.html').exists():
        raise SystemExit('demo has not been built')
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(site))
    with http.server.ThreadingHTTPServer((args.bind, args.port), handler) as server:
        host = network_host(args.bind)
        print(f'Listening on {args.bind}:{args.port}', flush=True)
        print(f'Network demo: http://{host}:{args.port}/snowflow_demo-fork/', flush=True)
        print('WebGPU on LAN HTTP requires HTTPS or a scoped browser secure-origin override.', flush=True)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


if __name__ == '__main__':
    main()
