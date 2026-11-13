"""Check browser harness contracts without pretending to execute a browser."""
import ast
import importlib.util
from pathlib import Path
import re
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('browser_baseline', ROOT / 'scripts/check-browser-baseline.py')
BROWSER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BROWSER)


class BrowserHarnessTests(unittest.TestCase):
    def test_dynamic_driver_port_is_parsed(self):
        with tempfile.TemporaryDirectory(prefix='snow-driver-') as directory:
            log = Path(directory) / 'driver.log'
            log.write_text('ChromeDriver was started successfully on port 43521.\n')
            with patch.object(BROWSER.time, 'sleep'):
                self.assertEqual(BROWSER.driver_address(log), 'http://127.0.0.1:43521')

    def test_worker_program_has_a_real_newline(self):
        with patch.object(BROWSER, 'request', return_value={'messages': ['1:r', '1:v', '1:o4:tick', '1:d']}) as request:
            BROWSER.worker_check('http://localhost', 'test')
            script = request.call_args.args[2]['script']
        literal = re.search(r"frame\(\[('(?:\\.|[^'])*')", script)[1]
        self.assertEqual(ast.literal_eval(literal), 'e := []E_VENT @\n[]E_KIND e')

    def test_out_of_order_worker_messages_fail(self):
        with patch.object(BROWSER, 'request', return_value={'messages': ['1:d', '1:r', '1:v', '1:o4:tick']}):
            with self.assertRaises(RuntimeError):
                BROWSER.worker_check('http://localhost', 'test')

    def test_worker_errors_fail_even_when_expected_messages_are_present(self):
        messages = ['1:r', '1:v', '1:o4:tick', '1:e5:error', '1:d']
        with patch.object(BROWSER, 'request', return_value={'messages': messages}):
            with self.assertRaises(RuntimeError):
                BROWSER.worker_check('http://localhost', 'test')


if __name__ == '__main__':
    unittest.main()
