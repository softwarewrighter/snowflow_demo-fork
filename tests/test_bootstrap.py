"""Dependency bootstrap must pin revisions and preserve developer changes."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('bootstrap', ROOT / 'scripts/bootstrap-xetal.py')
BOOTSTRAP = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BOOTSTRAP)


class CheckoutTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='snow-bootstrap-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.source = self.root / 'source'
        subprocess.run(['git', 'init', '-q', str(self.source)], check=True)
        self.command('config', 'user.name', 'Test')
        self.command('config', 'user.email', 'test@example.invalid')
        (self.source / 'pin.txt').write_text('one\n')
        self.command('add', 'pin.txt')
        self.command('-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-qm', 'first')
        self.first = self.command('rev-parse', 'HEAD')
        (self.source / 'pin.txt').write_text('two\n')
        self.command('commit', '-qam', 'second')
        self.destination = self.root / 'checkout'

    def command(self, *args):
        return subprocess.check_output(['git', '-C', str(self.source), *args], text=True).strip()

    def test_exact_revision_and_repeat(self):
        for _ in range(2):
            BOOTSTRAP.checkout(str(self.source), self.first, self.destination, offline=True)
        self.assertEqual((self.destination / 'pin.txt').read_text(), 'one\n')
        self.assertEqual((self.source / 'pin.txt').read_text(), 'two\n')

    def test_dirty_checkout_is_preserved(self):
        BOOTSTRAP.checkout(str(self.source), self.first, self.destination, offline=True)
        (self.destination / 'pin.txt').write_text('local changes\n')
        with self.assertRaisesRegex(RuntimeError, 'local changes'):
            BOOTSTRAP.checkout(str(self.source), self.first, self.destination, offline=True)
        self.assertEqual((self.destination / 'pin.txt').read_text(), 'local changes\n')

    def test_missing_revision_fails_without_resetting_checkout(self):
        BOOTSTRAP.checkout(str(self.source), self.first, self.destination, offline=True)
        with self.assertRaises((RuntimeError, subprocess.CalledProcessError)):
            BOOTSTRAP.checkout(str(self.source), '0' * 40, self.destination, offline=True)
        self.assertEqual((self.destination / 'pin.txt').read_text(), 'one\n')


if __name__ == '__main__':
    unittest.main()
