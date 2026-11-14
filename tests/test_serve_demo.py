"""LAN preview must bind externally and advertise a usable interface address."""
import importlib.util
import json
from pathlib import Path
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('serve_demo', ROOT / 'scripts/serve-demo.py')
SERVER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SERVER)


class ServeDemoTests(unittest.TestCase):
    def test_default_listens_on_all_interfaces_and_allows_loopback_override(self):
        self.assertEqual(SERVER.options(['--no-build']).bind, '0.0.0.0')
        self.assertEqual(SERVER.options(['--bind', '127.0.0.1']).bind, '127.0.0.1')

    def test_lan_address_is_preferred_over_docker(self):
        addresses = [{'addr_info': [{'family': 'inet', 'local': '172.17.0.1'}]},
                     {'addr_info': [{'family': 'inet', 'local': '192.168.1.178'}]}]
        with patch.object(SERVER.subprocess, 'check_output', return_value=json.dumps(addresses)):
            self.assertEqual(SERVER.network_host('0.0.0.0'), '192.168.1.178')

    def test_explicit_bind_address_is_advertised_without_detection(self):
        with patch.object(SERVER.subprocess, 'check_output') as command:
            self.assertEqual(SERVER.network_host('192.168.1.42'), '192.168.1.42')
            command.assert_not_called()

    def test_missing_ip_command_keeps_loopback_url_available(self):
        with patch.object(SERVER.subprocess, 'check_output', side_effect=FileNotFoundError):
            self.assertEqual(SERVER.network_host('0.0.0.0'), '127.0.0.1')


if __name__ == '__main__':
    unittest.main()
