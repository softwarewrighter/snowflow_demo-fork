#!/usr/bin/env python3
"""Build and serve the local X_eTaL snow demo; run from a normal terminal."""
import argparse
import functools
import http.server
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', type=int, default=8475)
    parser.add_argument('--no-build', action='store_true')
    args = parser.parse_args()
    if not args.no_build:
        subprocess.run([sys.executable, str(ROOT / 'scripts/build-demo.py')], check=True)
    site = ROOT / 'work/demo-site'
    if not (site / 'snowflow_demo-fork/index.html').exists():
        parser.error('demo has not been built')
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(site))
    with http.server.ThreadingHTTPServer(('127.0.0.1', args.port), handler) as server:
        print(f'Local demo: http://127.0.0.1:{args.port}/snowflow_demo-fork/', flush=True)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


if __name__ == '__main__':
    main()
