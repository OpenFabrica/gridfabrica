#!/usr/bin/env python3
"""Prepare pinned OFL Chinese faces in an external craft-fonts checkout.

Explicit packaging setup only: Cargo never downloads assets. Verify existing files rather
than overwriting them; add hashes and attribution to the shared font checkout for auditing.
"""

import argparse
import hashlib
import io
from pathlib import Path
import tempfile
import urllib.request
import zipfile


NOTO = "https://raw.githubusercontent.com/notofonts/noto-cjk/f8d157532fbfaeda587e826d4cd5b21a49186f7c/Serif/"
WENKAI = "https://raw.githubusercontent.com/lxgw/LxgwWenKai/29acb395f27f55110c2427e3ac2de1c12553cf6e/"
FONTS = [
    {
        "family": "Noto Serif CJK SC", "folder": "noto-serif-cjk-sc", "file": "NotoSerifCJKsc-Regular.otf",
        "url": NOTO + "OTF/SimplifiedChinese/NotoSerifCJKsc-Regular.otf",
        "hash": "2a2eae2628df83556c54018c41e20fa532c1b862c5256ae8b3f23feb918d12ca",
        "license_url": NOTO + "LICENSE",
        "license_hash": "6a73f9541c2de74158c0e7cf6b0a58ef774f5a780bf191f2d7ec9cc53efe2bf2",
        "author": "The Noto Project Authors",
    },
    {
        "family": "LXGW WenKai", "folder": "lxgw-wenkai", "file": "LXGWWenKai-Regular.ttf",
        "url": WENKAI + "fonts/TTF/LXGWWenKai-Regular.ttf",
        "hash": "39ad71264b588165b469e35e6afb162a378dacd1f95348160240ba9038ac3009",
        "license_url": WENKAI + "OFL.txt",
        "license_hash": "1a25e35da1031c6c3436fde545bb9cb5aca954e9873afe510c834b8b79bd21a0",
        "author": "LXGW and the Klee One Project Authors",
    },
    {
        "family": "Zhuque Fangsong (technical preview)", "folder": "zhuque-fangsong", "file": "ZhuqueFangsong-Regular.ttf",
        "url": "https://github.com/TrionesType/zhuque/releases/download/v0.212/ZhuqueFangsong-v0.212.zip",
        "archive_hash": "bb8b661a7643d2296a72d9d10530a00949419c4e527fb61783f73c2ba1a8c062",
        "hash": "558c62730844fe54ba220146ed62f859d4e2880188d92d985f8921c6e3743bc4",
        "license_url": "https://raw.githubusercontent.com/TrionesType/zhuque/005a8d0d620cb1fe2a576620a69daea3cc7477b4/LICENSE.txt",
        "license_hash": "66cc01ad4df62fca6936431a426104cb32a52ce8da49de9a7d643e02d7659043",
        "author": "TrionesType / JadeRipple Technology",
    },
]
LIMIT = 64 * 1024 * 1024


def verify(data: bytes, expected: str, name: str) -> bytes:
    if hashlib.sha256(data).hexdigest() != expected:
        raise ValueError(f"SHA-256 mismatch: {name}; refusing to install or overwrite")
    return data


def download(url: str) -> bytes:
    with urllib.request.urlopen(url, timeout=60) as response:
        data = response.read(LIMIT + 1)
    if len(data) > LIMIT:
        raise ValueError(f"Download exceeds 64 MiB: {url}")
    return data


def write_atomic(path: Path, data: bytes) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as tmp:
        temporary = Path(tmp.name)
        tmp.write(data)
    try:
        temporary.replace(path)
    finally:
        temporary.unlink(missing_ok=True)


def prepare(root: Path) -> None:
    root = root.resolve()
    app = Path(__file__).resolve().parents[1]
    if root == app or (app in root.parents and root != app / "craft-fonts"):
        raise ValueError("Font assets must live outside the gridfabrica repository")
    manifest_path = root / "fonts/manifest.txt"
    attribution_path = root / "ATTRIBUTION.md"
    manifest = manifest_path.read_text(encoding="utf-8")
    attribution = attribution_path.read_text(encoding="utf-8")
    pending = []
    rows = []
    for font in FONTS:
        relative = f"fonts/{font['folder']}/{font['file']}"
        path = root / relative
        if path.exists():
            data = verify(path.read_bytes(), font["hash"], relative)
        else:
            data = download(font["url"])
            if "archive_hash" in font:
                verify(data, font["archive_hash"], font["url"])
                with zipfile.ZipFile(io.BytesIO(data)) as archive:
                    if archive.getinfo(font["file"]).file_size > LIMIT:
                        raise ValueError("Font exceeds 64 MiB")
                    data = archive.read(font["file"])
            pending.append((path, verify(data, font["hash"], relative)))
        license_file = f"fonts/{font['folder']}/OFL.txt"
        license_path = root / license_file
        license_data = license_path.read_bytes() if license_path.exists() else download(font["license_url"])
        verify(license_data, font["license_hash"], license_file)
        if not license_path.exists():
            pending.append((license_path, license_data))
        row = f"{font['family']} | Regular | {relative} | Hans,Latn | OFL-1.1 | {license_file} | {font['hash']} | {font['url']}"
        existing = [line for line in manifest.splitlines() if relative in line]
        if existing and existing != [row]:
            raise ValueError(f"Conflicting manifest entry: {relative}")
        if not existing:
            manifest = manifest.rstrip() + "\n" + row + "\n"
        if f"`{relative}`" not in attribution:
            rows.append(f"| `{relative}` | {font['family']} Regular | {font['author']} | [Pinned source]({font['url']}) | OFL-1.1 (`{license_file}`) |")
    # Validate every download and existing entry before making any changes.
    for path, data in pending:
        write_atomic(path, data)
    if rows:
        attribution += "\n\n## gridfabrica Chinese font additions\n\n| File | Family / style | Author | Source (pinned) | Licence |\n|---|---|---|---|---|\n" + "\n".join(rows) + "\n"
    write_atomic(manifest_path, manifest.encode("utf-8"))
    write_atomic(attribution_path, attribution.encode("utf-8"))
    print(f"Verified {len(FONTS)} Chinese faces and licences in {root}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("craft_fonts", type=Path, help="external craft-fonts checkout")
    args = parser.parse_args()
    try:
        prepare(args.craft_fonts)
    except (OSError, ValueError, zipfile.BadZipFile, KeyError) as error:
        parser.exit(1, f"error: {error}\n")
