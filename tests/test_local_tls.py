"""Development certificates must match the LAN IP and retain their private key."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('local_tls', ROOT / 'scripts/local_tls.py')
TLS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TLS)


class LocalTlsTests(unittest.TestCase):
    def test_certificate_contains_lan_and_loopback_names_and_is_reused(self):
        with tempfile.TemporaryDirectory(prefix='snow-tls-') as directory:
            cert, key = TLS.certificate(Path(directory), '192.168.1.178')
            original = key.read_bytes()
            for host in ['192.168.1.178', '127.0.0.1']:
                subprocess.run(['openssl', 'x509', '-in', str(cert), '-noout', '-checkip', host], check=True, capture_output=True)
            subprocess.run(['openssl', 'x509', '-in', str(cert), '-noout', '-checkhost', 'localhost'], check=True, capture_output=True)
            self.assertEqual(key.stat().st_mode & 0o777, 0o600)
            self.assertEqual(TLS.certificate(Path(directory), '192.168.1.178'), (cert, key))
            self.assertEqual(key.read_bytes(), original)

    def test_invalid_ip_is_rejected(self):
        with tempfile.TemporaryDirectory(prefix='snow-tls-') as directory:
            with self.assertRaises(ValueError):
                TLS.certificate(Path(directory), 'invalid host')


if __name__ == '__main__':
    unittest.main()
