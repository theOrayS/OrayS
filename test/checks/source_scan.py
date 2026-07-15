"""Small source-scanning helpers shared by semantic static checks."""

from __future__ import annotations

from pathlib import Path


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore")


def function_block(text: str, name: str) -> str:
    marker = f"fn {name}"
    start = text.find(marker)
    if start < 0:
        return ""
    candidates = [
        pos
        for pos in (
            text.find("\nfn ", start + len(marker)),
            text.find("\npub fn ", start + len(marker)),
            text.find("\npub(super) fn ", start + len(marker)),
            text.find("\n    fn ", start + len(marker)),
            text.find("\n    pub fn ", start + len(marker)),
            text.find("\n    pub(super) fn ", start + len(marker)),
            text.find("\n#[", start + len(marker)),
        )
        if pos >= 0
    ]
    end = min(candidates) if candidates else len(text)
    return text[start:end]
