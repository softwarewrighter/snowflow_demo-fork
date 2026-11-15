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


def click(base, session, element_id):
    element = request(base, f'/session/{session}/element', {'using': 'css selector', 'value': '#' + element_id})
    key = element['element-6066-11e4-a52e-4f735466cecf']
    request(base, f'/session/{session}/element/{key}/click', {})


def interactions(base, session):
    wait(base, session, "const a=document.getElementById('app');if(a.dataset.state==='error')throw Error(document.getElementById('status').textContent);return a.dataset.state==='ready' && Number(a.dataset.chunks)===25")
    panels = evaluate(base, session, "return Array.from(document.querySelectorAll('.source-views pre[id$=\"-rendered\"]'),p=>({id:p.id,shared:p.classList.contains('rendered-source'),colors:['c-builtin','c-userfunc','c-number','c-string','c-doc'].map(c=>{const token=p.querySelector(c==='c-userfunc'?'.c-userfunc,.c-libfunc':'.'+c);return token?getComputedStyle(token).color:null})}))")
    assert len(panels) >= 5, panels
    for panel in panels:
        assert panel['shared'] and None not in panel['colors'] and len(set(panel['colors'])) == 5, panel
    order = evaluate(base, session, "return Array.from(document.querySelectorAll('.controls button')).filter(b=>['sweep','ribbon','bloom','crystallise','vortex'].includes(b.id)).map(b=>b.id)")
    assert order == ['sweep', 'ribbon', 'bloom', 'crystallise', 'vortex'], order
    click(base, session, 'start-sound')
    wait(base, session, "return document.getElementById('app').dataset.audio==='running'")
    position = "return JSON.parse(document.getElementById('app').dataset.position)"
    original = evaluate(base, session, position)
    mesh = evaluate(base, session, "return document.getElementById('app').dataset.mesh")
    time.sleep(0.5)
    assert evaluate(base, session, position) == original, 'idle camera bobbing'
    assert evaluate(base, session, "return document.getElementById('app').dataset.mesh") == mesh, 'idle terrain deformation'
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keydown',{key:'w'}))")
    time.sleep(8)
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keyup',{key:'w'}))")
    wait(base, session, "return document.getElementById('app').dataset.grounded==='true' && Number(document.getElementById('app').dataset.chunks)===25")
    assert evaluate(base, session, position)[2] < -15, 'old finite boundary remains'
    prints = evaluate(base, session, "return Number(document.getElementById('app').dataset.footprints)")
    steps = evaluate(base, session, "return Number(document.getElementById('app').dataset.steps)")
    assert prints > 0 and steps > 0, 'movement did not create footprints/footfalls'
    time.sleep(0.4)
    assert evaluate(base, session, "return Number(document.getElementById('app').dataset.steps)") == steps, 'footfalls continued while idle'
    (ROOT / 'work/demo-tracks-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    before_landings = evaluate(base, session, "return Number(document.getElementById('app').dataset.landings)")
    ground = evaluate(base, session, position)[1]
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keydown',{key:' '}))")
    time.sleep(0.2)
    assert evaluate(base, session, position)[1] > ground + 0.3, 'jump did not lift the camera'
    wait(base, session, f"return Number(document.getElementById('app').dataset.landings)>{before_landings} && document.getElementById('app').dataset.grounded==='true'")
    click(base, session, 'vortex')
    wait(base, session, "return Number(document.getElementById('app').dataset.vortex)===1536")
    (ROOT / 'work/demo-vortex-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    wait(base, session, "return Number(document.getElementById('app').dataset.vortex)===0 && Number(document.getElementById('app').dataset.particles)===0")
    for button in ['textures', 'shadows', 'sound']:
        label = evaluate(base, session, f"return document.getElementById('{button}').textContent")
        click(base, session, button)
        assert evaluate(base, session, f"return document.getElementById('{button}').textContent") != label
        click(base, session, button)
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keydown',{key:'4'}))")
    wait(base, session, "return Number(document.getElementById('app').dataset.crystals)===12")
    time.sleep(1.9)
    (ROOT / 'work/demo-crystals-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    edits_before = evaluate(base, session, "return Number(document.getElementById('app').dataset.edits)")
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keydown',{key:'1'}))")
    wait(base, session, f"return Number(document.getElementById('app').dataset.sweep)>0 && Number(document.getElementById('app').dataset.edits)>{edits_before}")
    (ROOT / 'work/demo-sweep-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    wait(base, session, "return Number(document.getElementById('app').dataset.sweep)===0")
    edits_before = evaluate(base, session, "return Number(document.getElementById('app').dataset.edits)")
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keydown',{key:'3'}))")
    wait(base, session, f"return Number(document.getElementById('app').dataset.bloom)>0 && Number(document.getElementById('app').dataset.particles)>0 && Number(document.getElementById('app').dataset.edits)>{edits_before}")
    time.sleep(0.5)
    (ROOT / 'work/demo-bloom-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    wait(base, session, "return Number(document.getElementById('app').dataset.bloom)===0 && Number(document.getElementById('app').dataset.particles)>0")
    wait(base, session, "return Number(document.getElementById('app').dataset.particles)===0")
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keydown',{key:'2'}))")
    wait(base, session, "return Number(document.getElementById('app').dataset.ribbon)>0 && document.getElementById('app').dataset.ribbonMode==='1'")
    time.sleep(0.4)
    (ROOT / 'work/demo-ribbon-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    evaluate(base, session, "window.dispatchEvent(new KeyboardEvent('keyup',{key:'2'}))")
    wait(base, session, "return document.getElementById('app').dataset.ribbonMode!=='1'")
    wait(base, session, "return document.getElementById('app').dataset.ribbonMode==='0'")
    click(base, session, 'ribbon')
    wait(base, session, "return document.getElementById('app').dataset.ribbonMode==='1'")
    evaluate(base, session, "window.dispatchEvent(new Event('blur'))")
    wait(base, session, "return document.getElementById('app').dataset.ribbonMode!=='1'")
    click(base, session, 'clear-tracks')
    wait(base, session, "return Number(document.getElementById('app').dataset.footprints)===0")
    click(base, session, 'reset')
    wait(base, session, "return Number(document.getElementById('app').dataset.chunks)===25 && JSON.parse(document.getElementById('app').dataset.position)[2]===8")
    assert evaluate(base, session, position) == original
    wait(base, session, "return Number(document.getElementById('app').dataset.crystals)===0")
    assert evaluate(base, session, "return document.getElementById('app').dataset.audio") == 'running'
    (ROOT / 'work/demo-screenshot.png').write_bytes(base64.b64decode(request(base, f'/session/{session}/screenshot')))
    return {'syntax_highlighting':'passed','gesture_audio_context':'passed (audibility requires human inspection)','idle_stability':'passed','streamed_walk':'passed','footprints':'passed','jump_landing':'passed','crystallise':'passed (appearance requires screenshot inspection)','sweep':'passed (appearance requires screenshot inspection)','bloom_particles':'passed (appearance requires screenshot inspection)','ribbon_hold_release_blur':'passed (appearance requires screenshot inspection)','visual_audio_toggles':'passed','vortex':'passed (appearance requires screenshot inspection)','reset':'passed','footprints_after_walk':prints}


def performance(base, session):
    results = {}
    for name, textures, shadows in [('off', False, False), ('textures', True, False), ('shadows', False, True), ('both', True, True)]:
        for control, enabled in [('textures', textures), ('shadows', shadows)]:
            evaluate(base, session, f"const b=document.getElementById('{control}');if(b.textContent.endsWith('on')!=={str(enabled).lower()})b.click()")
        click(base, session, 'reset')
        wait(base, session, "return Number(document.getElementById('app').dataset.chunks)===25 && JSON.parse(document.getElementById('app').dataset.position)[2]===8")
        time.sleep(0.3)
        script = """const app=document.getElementById('app');const initial=Number(app.dataset.frames);const done=arguments[arguments.length-1];const dt=[];let first=0,last=0;
        window.dispatchEvent(new KeyboardEvent('keydown',{key:'w'}));
        function frame(t){if(!first)first=t;if(last)dt.push(t-last);last=t;
        if(t-first<3000){requestAnimationFrame(frame);return;}
        window.dispatchEvent(new KeyboardEvent('keyup',{key:'w'}));
        const sorted=[...dt].sort((a,b)=>a-b);const mean=dt.reduce((a,b)=>a+b,0)/dt.length;
        done({app_frames:Number(app.dataset.frames)-initial,frames:dt.length,mean_ms:mean,p95_ms:sorted[Math.floor(sorted.length*.95)],fps:1000/mean,over_25ms:dt.filter(x=>x>25).length});}
        requestAnimationFrame(frame);"""
        results[name] = request(base, f'/session/{session}/execute/async', {'script': script, 'args': []})
        assert results[name]['app_frames'] > 0, name
        assert evaluate(base, session, "return document.getElementById('app').dataset.state") == 'ready'
    baseline = results['off']['p95_ms']
    return {'method':'three-second RAF intervals per mode, walking from reset through one chunk boundary; not GPU timestamps','modes':results,'both_p95_increase_percent':100*(results['both']['p95_ms']/baseline-1),'target_percent':10}


def browser(base, url, profile, measure=False):
    options = {'args': ['--headless=new', '--no-sandbox', '--disable-dev-shm-usage', '--enable-unsafe-webgpu', '--no-proxy-server', f'--user-data-dir={profile}']}
    created = request(base, '/session', {'capabilities': {'alwaysMatch': {'browserName': 'chrome', 'goog:chromeOptions': options, 'goog:loggingPrefs': {'browser': 'ALL'}}}})
    session = created['sessionId']
    try:
        request(base, f'/session/{session}/window/rect', {'width': 1280, 'height': 1000})
        request(base, f'/session/{session}/url', {'url': url})
        result = interactions(base, session)
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
