#!/usr/bin/env python3
"""Headless WebGPU demo validation; run in a terminal permitting local sockets."""
import base64
import functools
import http.server
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import threading
import time

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('baseline', ROOT / 'scripts/check-browser-baseline.py')
BASELINE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BASELINE)
request = BASELINE.request


def evaluate(base, session, script):
    return request(base, f'/session/{session}/execute/sync', {'script': script, 'args': []})


def wait(base, session, condition):
    for _ in range(200):
        result = evaluate(base, session, condition)
        if result:
            return result
        time.sleep(0.1)
    raise RuntimeError('timed out waiting for demo state')


def interactions(base, session):
    updates = "Number(document.getElementById('app').dataset.updates)"
    ready = "const a=document.getElementById('app');if(a.dataset.state==='error')throw Error(document.getElementById('status').textContent);return a.dataset.state==='ready' && Number(a.dataset.updates)>1"
    wait(base, session, ready)
    evaluate(base, session, "document.getElementById('pause').click()")
    time.sleep(0.3)
    before = evaluate(base, session, 'return ' + updates)
    time.sleep(0.25)
    assert evaluate(base, session, 'return ' + updates) == before, 'pause did not settle'
    hashes = []
    for button in ['reset', 'raise', 'reset']:
        old = evaluate(base, session, 'return ' + updates)
        evaluate(base, session, f"document.getElementById('{button}').click()")
        wait(base, session, f'return {updates} > {old}')
        hashes.append(evaluate(base, session, "return document.getElementById('app').dataset.mesh"))
    assert hashes[0] != hashes[1] and hashes[0] == hashes[2], f'input/reset failed: {hashes}'
    position = "return document.getElementById('app').dataset.position"
    start = evaluate(base, session, position)
    evaluate(base, session, "document.getElementById('pause').click();window.dispatchEvent(new KeyboardEvent('keydown',{key:'w'}))")
    time.sleep(0.5)
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keyup',{key:'w'}));document.getElementById('pause').click()")
    moved = evaluate(base, session, position)
    assert moved != start, 'walking did not change camera position'
    evaluate(base, session, "document.getElementById('reset').click()")
    wait(base, session, "return document.getElementById('app').dataset.position === " + json.dumps(start))
    screenshot = request(base, f'/session/{session}/screenshot')
    (ROOT / 'work/demo-screenshot.png').write_bytes(base64.b64decode(screenshot))
    return {'pause': 'passed', 'walking_and_reset': 'passed', 'input_and_reset': 'passed', 'mesh_hashes': hashes}


def browser(base, url, profile):
    options = {'args': ['--headless=new', '--no-sandbox', '--disable-dev-shm-usage', '--enable-unsafe-webgpu', '--no-proxy-server', f'--user-data-dir={profile}']}
    created = request(base, '/session', {'capabilities': {'alwaysMatch': {'browserName': 'chrome', 'goog:chromeOptions': options, 'goog:loggingPrefs': {'browser': 'ALL'}}}})
    session = created['sessionId']
    try:
        request(base, f'/session/{session}/window/rect', {'width': 1280, 'height': 1000})
        request(base, f'/session/{session}/url', {'url': url})
        result = interactions(base, session)
        logs = request(base, f'/session/{session}/log', {'type': 'browser'})
        assert not any(item['level'] == 'SEVERE' for item in logs), logs
        request(base, f'/session/{session}/goog/cdp/execute', {'cmd': 'Page.addScriptToEvaluateOnNewDocument', 'params': {'source': "Object.defineProperty(Navigator.prototype,'gpu',{get:()=>undefined})"}})
        request(base, f'/session/{session}/url', {'url': url})
        wait(base, session, "return document.getElementById('app').dataset.state==='error'")
        result['unsupported_device'] = evaluate(base, session, "return document.getElementById('status').textContent")
        assert 'WebGPU is unavailable' in result['unsupported_device']
        return dict(result, status='passed', capabilities=created['capabilities'], browser_logs=logs)
    finally:
        request(base, f'/session/{session}', method='DELETE')


def main():
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(ROOT / 'work/demo-site'))
    with http.server.ThreadingHTTPServer(('127.0.0.1', 0), handler) as server:
        threading.Thread(target=server.serve_forever, daemon=True).start()
        with tempfile.TemporaryDirectory(prefix='snow-demo-browser-') as temporary:
            log = Path(temporary) / 'driver.log'
            with log.open('w') as output:
                driver = subprocess.Popen(['chromedriver', '--port=0'], stdout=output, stderr=output)
                try:
                    base = BASELINE.driver_address(log)
                    result = browser(base, f'http://127.0.0.1:{server.server_port}/snowflow_demo-fork/', Path(temporary) / 'profile')
                    (ROOT / 'work/demo-browser-result.json').write_text(json.dumps(result, indent=2) + '\n')
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
        (ROOT / 'work/demo-browser-result.json').write_text(json.dumps(result, indent=2) + '\n')
        print(json.dumps(result, indent=2))
        raise SystemExit(1)
