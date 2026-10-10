# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example files, presets) is listed here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset file is missing from this table.

**Policy (mandatory):** GridFabrica contains **no Microsoft, Adobe, Avid or Autodesk iconography, images, artwork, fonts, templates, themes, table styles or sample files**. Every asset is original work by GridFabrica contributors or third-party material under an open licence (OSI open source, public domain / CC0, or Creative Commons that allows redistribution). Screenshots of Microsoft software are never committed. Font files are not added here: shared fonts live in [OpenFabrica/craft-fonts](https://github.com/OpenFabrica/craft-fonts), an optional build input (`CRAFT_FONTS_DIR`). ArtCraft trademark artwork is excluded from this fork; its original terms are preserved in `docs/brand/LICENSE-brand.txt`.

Generated-in-code art is original and has no file to list: the UI icon set (`crates/ui-egui/src/icons.rs`), themes (`crates/engine/src/cmd/view.rs`), cell styles (`crates/engine/src/cmd/format.rs`), table styles (`crates/engine/src/tables.rs`), chart palettes and the sample workbooks (`crates/engine/src/sample.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `docs/brand/LICENSE-brand.txt` | (licence text) | craftrules | — | |
| `assets/app-icon/gridfabrica.svg` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | PLACEHOLDER app icon (ledger page); replaced by the owner's creature art later |
| `packaging/macos/dmg/background.svg` | @XusBadia | original (hand-written SVG, derived from the GridFabrica app icon `assets/app-icon/gridfabrica.svg`, referenced via `<image>`, not copied; text is outlined glyph paths (title: Ubuntu Light from egui default fonts, Ubuntu Font Licence 1.0; existing labels: Inter / JetBrains Mono, OFL 1.1); `packaging/macos/dmg/generate.py` renders it to `background.tiff`) | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | GridFabrica macOS DMG window background for the Finder window; added 2026-10-08 |
| `packaging/macos/dmg/background.tiff` | @XusBadia | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | 1x + 2x HiDPI TIFF for the macOS DMG window; includes the app icon (see its row) and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `assets/app-icon/gridfabrica-1024.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, rendered from gridfabrica.svg |
| `assets/app-icon/gridfabrica-macos-512.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, macOS grid render |
| `assets/app-icon/gridfabrica.icns` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, macOS bundle icon |
| `assets/app-icon/gridfabrica.ico` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Windows exe icon |
| `assets/app-icon/hicolor/16x16/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/24x24/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/32x32/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/48x48/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/64x64/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/256x256/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/512x512/apps/ai.storyteller.gridfabrica.png` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/scalable/apps/ai.storyteller.gridfabrica.svg` | GridCraft contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, copy of gridfabrica.svg |
| `assets/app-icon/README.md` | GridCraft contributors | original | MIT OR Apache-2.0 | Icon documentation |
| `assets/app-icon/LICENSE.txt` | (licence text) | original | — | Licence of the app icon files |
| `docs/images/hero-sales.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/budget.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/grades-formulas.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/dark-insert.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/pivot.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/format-chart.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/page-break.png` | GridCraft contributors | screenshot of GridFabrica rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |

## Optional Chinese font input

Native builds made with `CRAFT_FONTS_DIR` embed Noto Sans CJK SC Regular; web builds publish it
as an independent font resource. The files come from
[`OpenFabrica/craft-fonts`](https://github.com/OpenFabrica/craft-fonts), authored by the Noto Project
contributors under SIL Open Font License 1.1. The font, pinned source and licence are recorded
in that repository's `fonts/manifest.txt` and `ATTRIBUTION.md`; no font asset lives here.
Additional Chinese faces are prepared by `packaging/chinese-fonts.py`, which pins and verifies
Noto Serif CJK SC (Noto Project Authors), LXGW WenKai (LXGW and Klee One Project Authors), and
Zhuque Fangsong (TrionesType), all under OFL-1.1. Font files and their attribution remain in
craft-fonts. Release packages include the OFL licence text for every font face, embedded or served separately.
