"""Only the intended static artifact may be prepared for public Pages hosting."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('package_pages', ROOT / 'scripts/package-pages.py')
PAGES = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PAGES)


class PackageTests(unittest.TestCase):
    def test_static_files_and_build_identity_are_staged(self):
        with tempfile.TemporaryDirectory(prefix='snow-pages-') as directory:
            source, target = Path(directory) / 'site', Path(directory) / 'public'
            source.mkdir()
            (source / 'index.html').write_text('<html>demo</html>')
            (source / 'demo.wasm').write_bytes(b'wasm')
            PAGES.stage(source, target, 'a' * 40)
            self.assertEqual((target / 'demo.wasm').read_bytes(), b'wasm')
            self.assertTrue((target / '.nojekyll').exists())
            self.assertIn('a' * 40, (target / 'build-info.json').read_text())

    def test_private_key_is_rejected_before_copying(self):
        with tempfile.TemporaryDirectory(prefix='snow-pages-') as directory:
            source, target = Path(directory) / 'site', Path(directory) / 'public'
            source.mkdir()
            (source / 'index.html').write_text('demo')
            (source / 'private.key').write_text('not for publication')
            with self.assertRaises(ValueError):
                PAGES.stage(source, target, 'b' * 40)
            self.assertFalse(target.exists())


if __name__ == '__main__':
    unittest.main()
