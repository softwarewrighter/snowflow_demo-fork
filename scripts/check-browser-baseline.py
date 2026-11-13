#!/usr/bin/env python3
"""Exercise the built upstream page and event worker through ChromeDriver."""
import functools
import http.server
import json
from pathlib import Path
import re
import subprocess
import tempfile
import threading
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
OPENER = urllib.request.build_opener(urllib.request.ProxyHandler({}))


def request(base, path, data=None, method=None):
    body = json.dumps(data).encode() if data is not None else None
    req = urllib.request.Request(base + path, body, {'Content-Type': 'application/json'}, method=method)
    with OPENER.open(req, timeout=45) as response:
        result = json.load(response)['value']
    if isinstance(result, dict) and result.get('error'):
        raise RuntimeError(result)
    return result


def worker_check(base, session):
    script = r'''
const done = arguments[arguments.length - 1];
const frame = fields => fields.map(s => `${new TextEncoder().encode(s).length}:${s}`).join('');
const messages = [];
const worker = new Worker('/snowflow_demo-fork/xetal-runner_loader.js');
const timer = setTimeout(() => finish({error: 'worker timed out', messages}), 30000);
function finish(result) { clearTimeout(timer); worker.terminate(); done(result); }
worker.onerror = event => finish({error: event.message, messages});
worker.onmessage = event => {
    const value = event.data;
    messages.push(value);
    if (value === '1:r') worker.postMessage(frame(['e := []E_VENT @\n[]E_KIND e', '1', 'r', '-']));
    if (value === '1:v') worker.postMessage(frame(['line', 'tick 0.016']));
    if (value === '1:d') finish({messages});
};
'''
    result = request(base, f'/session/{session}/execute/async', {'script': script, 'args': []})
    if result.get('error') or result.get('messages') != ['1:r', '1:v', '1:o4:tick', '1:d']:
        raise RuntimeError(f'worker event check failed: {result}')
    return result


def browser(base, url, profile):
    options = {'args': ['--headless=new', '--no-sandbox', '--disable-dev-shm-usage', '--no-proxy-server', f'--user-data-dir={profile}']}
    created = request(base, '/session', {'capabilities': {'alwaysMatch': {'browserName': 'chrome', 'goog:chromeOptions': options, 'goog:loggingPrefs': {'browser': 'ALL'}}}})
    session = created['sessionId']
    try:
        request(base, f'/session/{session}/timeouts', {'script': 35000})
        request(base, f'/session/{session}/url', {'url': url})
        for _ in range(100):
            ready = request(base, f'/session/{session}/execute/sync', {'script': "return !!document.querySelector('textarea')", 'args': []})
            if ready:
                break
            time.sleep(0.1)
        if not ready:
            raise RuntimeError('upstream editor failed to mount')
        result = worker_check(base, session)
        logs = request(base, f'/session/{session}/log', {'type': 'browser'})
        if any(item['level'] == 'SEVERE' for item in logs):
            raise RuntimeError(f'browser errors: {logs}')
        return dict(result, browser_logs=logs, capabilities=created['capabilities'])
    finally:
        request(base, f'/session/{session}', method='DELETE')


def driver_address(log):
    for _ in range(100):
        found = re.search(r'started successfully on port (\d+)', log.read_text())
        if found:
            return f'http://127.0.0.1:{found[1]}'
        time.sleep(0.1)
    raise RuntimeError(f'ChromeDriver failed to start: {log.read_text()}')


def main():
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(ROOT / 'work/baseline-site'))
    with http.server.ThreadingHTTPServer(('127.0.0.1', 0), handler) as server:
        threading.Thread(target=server.serve_forever, daemon=True).start()
        with tempfile.TemporaryDirectory(prefix='snow-browser-') as temporary:
            log = Path(temporary) / 'driver.log'
            with log.open('w') as output:
                driver = subprocess.Popen(['chromedriver', '--port=0'], stdout=output, stderr=output)
                try:
                    base = driver_address(log)
                    result = browser(base, f'http://127.0.0.1:{server.server_port}/snowflow_demo-fork/', Path(temporary) / 'profile')
                    result['status'] = 'passed'
                    (ROOT / 'work/browser-baseline-result.json').write_text(json.dumps(result, indent=2) + '\n')
                    print(json.dumps(result, indent=2))
                finally:
                    driver.terminate()
                    driver.wait(timeout=10)
                    server.shutdown()


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        result = {'status': 'failed', 'error_type': type(error).__name__, 'error': str(error)}
        (ROOT / 'work').mkdir(exist_ok=True)
        (ROOT / 'work/browser-baseline-result.json').write_text(json.dumps(result, indent=2) + '\n')
        print(json.dumps(result, indent=2))
        raise SystemExit(1)
