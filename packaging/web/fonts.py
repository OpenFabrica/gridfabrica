#!/usr/bin/env python3
"""Trunk post-build hook: copy hash-addressed font files and licences into its staging dir.

Uses an already prepared external craft-fonts checkout. No fonts are downloaded here, and
every font is verified against its manifest before copying. Optional builds allow a partial
pack; CRAFT_FONTS_REQUIRED=1 requires all four faces.
"""
import hashlib
import os
from pathlib import Path
import shutil
import sys

FILES = [
    "fonts/noto-sans-cjk-sc/NotoSansCJKsc-Regular.otf",
    "fonts/noto-serif-cjk-sc/NotoSerifCJKsc-Regular.otf",
    "fonts/lxgw-wenkai/LXGWWenKai-Regular.ttf",
    "fonts/zhuque-fangsong/ZhuqueFangsong-Regular.ttf",
]


def copy_fonts() -> None:
    app = Path(__file__).resolve().parents[2]
    configured = os.environ.get("CRAFT_FONTS_DIR")
    root = Path(configured) if configured else next(
        (p for p in [app / "craft-fonts", app.parent / "craft-fonts"] if (p / FILES[0]).is_file()), None
    )
    required = "CRAFT_FONTS_REQUIRED" in os.environ
    if root is None:
        if required:
            raise ValueError("CRAFT_FONTS_REQUIRED needs a prepared craft-fonts checkout")
        return
    staging = Path(os.environ["TRUNK_STAGING_DIR"]) / "fonts"
    # Trunk creates a fresh staging directory for every build.
    staging.mkdir(parents=True, exist_ok=True)
    entries = {}
    for line in (root / "fonts/manifest.txt").read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        fields = line.split(" | ")
        if len(fields) == 8:
            entries[fields[2]] = fields
    for relative in FILES:
        path = root / relative
        if not path.is_file():
            if required:
                raise ValueError(f"Missing required Chinese font: {path}")
            continue
        fields = entries.get(relative)
        if fields is None:
            raise ValueError(f"Font absent from manifest: {relative}")
        if path.stat().st_size > 64 * 1024 * 1024:
            raise ValueError(f"Font exceeds 64 MiB: {relative}")
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != fields[6]:
            raise ValueError(f"Font SHA-256 mismatch: {relative}")
        destination = staging / f"{path.stem}-{digest}{path.suffix}"
        shutil.copyfile(path, destination)
        shutil.copyfile(root / fields[5], staging / f"OFL-{path.parent.name}.txt")
    print(f"Copied independent Chinese font resources to {staging}")


if __name__ == "__main__":
    try:
        copy_fonts()
    except (OSError, ValueError, KeyError) as error:
        sys.exit(f"error: {error}")
