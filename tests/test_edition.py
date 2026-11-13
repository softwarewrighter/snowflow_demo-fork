"""All new component crates must use the requested Rust 2024 edition."""
from pathlib import Path
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[1]


class EditionTests(unittest.TestCase):
    def test_workspace_and_crate_editions(self):
        workspaces = sorted((ROOT / 'components').glob('*/Cargo.toml'))
        self.assertTrue(workspaces)
        for manifest in workspaces:
            with self.subTest(manifest=manifest):
                workspace = tomllib.loads(manifest.read_text())['workspace']
                self.assertEqual(workspace['package']['edition'], '2024')
                for member in workspace['members']:
                    crate = tomllib.loads((manifest.parent / member / 'Cargo.toml').read_text())
                    self.assertEqual(crate['package']['edition'], {'workspace': True})


if __name__ == '__main__':
    unittest.main()
