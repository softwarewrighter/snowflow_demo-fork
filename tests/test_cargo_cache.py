"""The local extraction cache must preserve its read-only source and own downloads."""
import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('cargo_cache', ROOT / 'scripts/cargo_cache.py')
CACHE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CACHE)


class CacheTests(unittest.TestCase):
    def test_repeated_preparation_retains_downloads_and_reuses_sources(self):
        with tempfile.TemporaryDirectory(prefix='snow-cargo-') as directory:
            source, target = Path(directory) / 'source', Path(directory) / 'target'
            package = source / 'registry/src/index/example-1.0'
            package.mkdir(parents=True)
            (source / 'registry/cache/index').mkdir(parents=True)
            (source / 'registry/index').mkdir()
            (source / 'registry/cache/index/example.crate').write_bytes(b'archive')
            for _ in range(2):
                CACHE.prepare(source, target)
            self.assertTrue((target / 'registry/src/index/example-1.0').is_symlink())
            self.assertFalse((target / 'registry/cache').is_symlink())
            download = target / 'registry/cache/index/new.crate'
            download.write_bytes(b'local')
            CACHE.prepare(source, target)
            self.assertEqual(download.read_bytes(), b'local')
            self.assertFalse((source / 'registry/cache/index/new.crate').exists())

    def test_same_source_and_target_does_not_recurse(self):
        with tempfile.TemporaryDirectory(prefix='snow-cargo-') as directory:
            self.assertEqual(CACHE.prepare(directory, directory), Path(directory).resolve())
