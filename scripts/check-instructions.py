#!/usr/bin/env python3
"""Validate owned instructions without rewriting the generated agentrail block."""
from pathlib import Path
import re

root = Path(__file__).resolve().parents[1]
text = (root / "CLAUDE.md").read_text()
owned = re.sub(
    r"<!-- agentrail:global:start.*?<!-- agentrail:global:end -->",
    "",
    text,
    flags=re.DOTALL,
)
assert all(32 <= ord(char) < 127 or char in "\n\r\t" for char in owned)
assert (root / "AGENTS.md").is_symlink()
assert (root / "AGENTS.md").resolve() == root / "CLAUDE.md"
print("instructions: owned text is ASCII and AGENTS.md links to CLAUDE.md")
