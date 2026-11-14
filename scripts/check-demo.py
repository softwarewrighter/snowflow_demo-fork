#!/usr/bin/env python3
"""Headless WebGPU demo validation; run in a terminal permitting local sockets."""
import argparse
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
    colors = evaluate(base, session, "return ['c-builtin','c-userfunc','c-number','c-string','c-doc'].map(c=>{const e=document.querySelector('#source-rendered .'+c);if(!e)throw Error('Missing syntax class '+c);return getComputedStyle(e).color})")
    assert len(set(colors)) == 5, f'syntax colors are missing or indistinguishable: {colors}'
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
    return {'syntax_highlighting': 'passed', 'pause': 'passed', 'walking_and_reset': 'passed', 'input_and_reset': 'passed', 'mesh_hashes': hashes}


def effects(base, session):
    for button in ['textures', 'shadows']:
        original = evaluate(base, session, f"return document.getElementById('{button}').textContent")
        evaluate(base, session, f"document.getElementById('{button}').click()")
        assert evaluate(base, session, f"return document.getElementById('{button}').textContent") != original
        evaluate(base, session, f"document.getElementById('{button}').click()")
    position = "return JSON.parse(document.getElementById('app').dataset.position)"
    baseline = evaluate(base, session, position)[1]
    evaluate(base, session, "document.getElementById('pause').click();window.dispatchEvent(new KeyboardEvent('keydown',{key:' '}))")
    time.sleep(0.2)
    assert evaluate(base, session, position)[1] > baseline + 0.3, 'jump did not raise the camera'
    time.sleep(1.2)
    evaluate(base, session, "document.getElementById('pause').click();document.getElementById('reset').click()")
    return {'visual_toggles': 'passed', 'jump': 'passed'}


def performance(base, session):
    results = {}
    evaluate(base, session, "if(document.getElementById('pause').textContent==='Resume')document.getElementById('pause').click()")
    for name, textures, shadows in [('off', False, False), ('textures', True, False), ('shadows', False, True), ('both', True, True)]:
        for control, enabled in [('textures', textures), ('shadows', shadows)]:
            evaluate(base, session, f"const b=document.getElementById('{control}');if(b.textContent.endsWith('on')!=={str(enabled).lower()})b.click()")
        evaluate(base, session, "document.getElementById('reset').click()")
        time.sleep(0.3)
        script = """const app=document.getElementById('app');const initial=Number(app.dataset.frames);const done=arguments[arguments.length-1];const dt=[];let first=0,last=0;
        function frame(t){if(!first)first=t;if(last)dt.push(t-last);last=t;
        if(t-first<3000){requestAnimationFrame(frame);return;}
        const sorted=[...dt].sort((a,b)=>a-b);const mean=dt.reduce((a,b)=>a+b,0)/dt.length;
        done({app_frames:Number(app.dataset.frames)-initial,frames:dt.length,mean_ms:mean,p95_ms:sorted[Math.floor(sorted.length*.95)],fps:1000/mean,over_25ms:dt.filter(x=>x>25).length});}
        requestAnimationFrame(frame);"""
        results[name] = request(base, f'/session/{session}/execute/async', {'script': script, 'args': []})
        assert results[name]['app_frames'] > 0, f'application stopped rendering in {name} mode'
        assert evaluate(base, session, "return document.getElementById('app').dataset.state") == 'ready'
    return {'method': 'three seconds of RAF intervals per mode, active terrain; no GPU timestamp query', 'modes': results}


def browser(base, url, profile, measure=False):
    options = {'args': ['--headless=new', '--no-sandbox', '--disable-dev-shm-usage', '--enable-unsafe-webgpu', '--no-proxy-server', f'--user-data-dir={profile}']}
    created = request(base, '/session', {'capabilities': {'alwaysMatch': {'browserName': 'chrome', 'goog:chromeOptions': options, 'goog:loggingPrefs': {'browser': 'ALL'}}}})
    session = created['sessionId']
    try:
        request(base, f'/session/{session}/window/rect', {'width': 1280, 'height': 1000})
        request(base, f'/session/{session}/url', {'url': url})
        result = interactions(base, session)
        result.update(effects(base, session))
        if measure:
            result['frame_comparison'] = performance(base, session)
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
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--performance', action='store_true', help='compare frame intervals with effects off/on')
    args = parser.parse_args()
    handler = functools.partial(http.server.SimpleHTTPRequestHandler, directory=str(ROOT / 'work/demo-site'))
    with http.server.ThreadingHTTPServer(('127.0.0.1', 0), handler) as server:
        threading.Thread(target=server.serve_forever, daemon=True).start()
        with tempfile.TemporaryDirectory(prefix='snow-demo-browser-') as temporary:
            log = Path(temporary) / 'driver.log'
            with log.open('w') as output:
                driver = subprocess.Popen(['chromedriver', '--port=0'], stdout=output, stderr=output)
                try:
                    base = BASELINE.driver_address(log)
                    result = browser(base, f'http://127.0.0.1:{server.server_port}/snowflow_demo-fork/', Path(temporary) / 'profile', args.performance)
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
