<h1 align="center">GridFabrica</h1>

<p align="center">
  <b>Spreadsheets and data; an open-source, clean-room reimplementation of Microsoft Excel, rebuilt in pure Rust.</b>
</p>

<p align="center">
  A fast, open-source, clean-room take on the Excel workflow. It runs natively on macOS,
  Windows, Linux and BSD, and in the browser via WebAssembly.<br>
  <i>Maintained by OpenFabrica.</i>
</p>

<p align="center">
  <img alt="Written in Rust" src="https://img.shields.io/badge/written%20in-Rust-147a40?style=flat-square&logo=rust&logoColor=white">
  <img alt="Runs on macOS, Windows, Linux, BSD and the web" src="https://img.shields.io/badge/runs%20on-macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20BSD%20%C2%B7%20Web-1f9d55?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-147a40?style=flat-square">
  <img alt="Agent-drivable over MCP" src="https://img.shields.io/badge/agents-MCP-1f9d55?style=flat-square">
</p>

<p align="center">
  <a href="https://github.com/OpenFabrica/gridfabrica"><b>GridFabrica on GitHub</b></a> ·
  <a href="https://github.com/OpenFabrica/gridfabrica/issues">Report an issue</a>
</p>

<br>

<p align="center">
  <img src="docs/images/hero-sales.png" alt="GridFabrica showing a quarterly sales dashboard: a styled table with banded rows, filter buttons, a totals row, data bars, arrow icons and sparklines, next to a clustered column chart and a pie chart" width="100%">
  <br><sub><b>Quarterly Sales Dashboard</b>: a table with a totals row, conditional formatting, sparklines and two live charts — built entirely from GridFabrica commands (<code>File → New → Sample</code>).</sub>
</p>

<p align="center">
  <a href="#a-look-around">A look around</a> ·
  <a href="#why-gridfabrica">Why GridFabrica</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#for-agents-mcp-cli-and-the-control-channel">For agents</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#roadmap">Roadmap</a> ·
  <a href="#license-and-credits">License and credits</a>
</p>

## A look around

<table>
<tr>
<td width="50%" valign="top"><img src="docs/images/budget.png" alt="A household budget with accounting number formats, red highlights for overspent categories, a green-yellow-red colour scale on the percent-used column, a totals row with a double border, and a horizontal bar chart comparing budget with actual" width="100%"><p align="center"><sub><b>Formatting.</b> Accounting formats, cell styles, conditional rules and colour scales, frozen headers and a bar chart.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/grades-formulas.png" alt="A gradebook table with averages and letter grades computed with IFS, green highlights on A grades and data bars, with the Formulas ribbon tab open showing the function library" width="100%"><p align="center"><sub><b>Formulas.</b> 500+ functions, dynamic arrays, LET and LAMBDA, structured table references.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/dark-insert.png" alt="The sales dashboard in dark mode with the Insert ribbon tab open, showing PivotTable, Table, Pictures, Shapes, chart, sparkline, link, comment, text box and symbol buttons" width="100%"><p align="center"><sub><b>Dark mode</b> and the Insert tab: tables, charts, sparklines, pictures, shapes, links and comments.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/pivot.png" alt="A PivotTable summarising revenue by region and product with a Channel report filter, grand totals and currency formatting, with the PivotTable Fields pane open on the right showing ticked fields and the Filters, Columns, Rows and Values areas" width="100%"><p align="center"><sub><b>PivotTables.</b> Field list, filters, layouts, date grouping and show-values-as, saved to and read from XLSX.</sub></p></td>
</tr>
<tr>
<td width="50%" valign="top"><img src="docs/images/format-chart.png" alt="The sales dashboard with the column chart selected: resize handles on the chart, its source data outlined in the table, the Chart Design tab open and the Format Chart Area pane on the right" width="100%"><p align="center"><sub><b>Charts.</b> Seventeen chart types, Chart Design tab and a Format pane; selecting a chart outlines its source data.</sub></p></td>
<td width="50%" valign="top"><img src="docs/images/page-break.png" alt="The household budget in Page Break Preview: the print area outlined in blue with a dashed page break, Page 1 and Page 2 watermarks, and the area outside the print range greyed out" width="100%"><p align="center"><sub><b>Page Break Preview</b>, print settings, and PDF export and printing.</sub></p></td>
</tr>
</table>

## Why GridFabrica

- **Familiar.** Excel's ribbon, formula bar, Name Box, sheet tabs, keyboard shortcuts, fill
  handle, point-mode formula entry with coloured references, Format Cells, Paste Special,
  conditional formatting, tables, charts and more. If you know Excel, you already know GridFabrica.
- **Compatible.** XLSX is the native format: styles, themes, formulas (including dynamic arrays),
  tables, conditional formats, validation, comments, hyperlinks, charts, pictures, sparklines,
  PivotTables and print settings round-trip. CSV and TSV too.
- **Capable.** A dependency-graph calculation engine with dynamic arrays and spilling,
  500+ worksheet functions plus LET, LAMBDA, MAP, REDUCE, SCAN, BYROW, BYCOL and MAKEARRAY,
  structured table references, Excel's full number-format language, sort and AutoFilter,
  Flash Fill, data validation, outlining and subtotals.
- **Fast.** Copy-on-write workbooks (undo snapshots are nearly free), incremental recalculation,
  and a virtualised grid that only lays out what's on screen.
- **Agent-native.** Every ribbon button, menu item, dialog, keystroke and mouse gesture is a
  command that agents can run over a JSON control channel, an **MCP server**, or the CLI.
- **Everywhere.** One Rust codebase for desktop and the web. No subscription, no licence server,
  no telemetry.

## Quick start

```sh
cargo run --release -p gridfabrica                              # desktop app
cargo run --release -p gridfabrica -- --sample sales            # open a sample (sales, budget, grades)
cargo run --release -p gridfabrica -- book.xlsx                 # open a workbook
cargo run --release -p gridfabrica -- --sample sales --control 7979   # + JSON control channel
cargo xtask ci                                                 # fmt, clippy, tests, assets, layering, wasm
```

### Web

```sh
cd apps/gridfabrica-web && trunk build --release     # → dist/web (serve it with any static server)
cd apps/gridfabrica-web && trunk serve --release     # http://127.0.0.1:8771  (?sample=sales opens a sample)
```

You need [trunk](https://trunkrs.dev) and the `wasm32-unknown-unknown` target. Open uses the
browser's file picker (dropping files works too) and Save downloads the workbook.

The title-bar language picker switches between **English** and **简体中文**. Desktop and web
remember the choice. Chinese web builds use the optional `CRAFT_FONTS_DIR` input; see
[interface localization](docs/localization.md).

## For agents: MCP, CLI and the control channel

```sh
claude mcp add gridfabrica -- gridfabrica-cli mcp                 # headless MCP server
gridfabrica-cli mcp --connect 7979                               # MCP bridged to a running window
gridfabrica-cli eval '=XLOOKUP("b",{"a","b"},{1,2})'             # → 2
gridfabrica-cli run --sample budget --cmd 'home.bold={"range":"B4:F4"}' --out budget.xlsx
gridfabrica-cli cat budget.xlsx --range B4:F15                   # aligned text table
gridfabrica-cli commands --search chart                          # every command and its params
```

The MCP server offers tools such as `read_range`, `write_range`, `set_cell`, `evaluate_formula`,
`format_range`, `insert_chart`, `create_table`, `sort_range`, `filter` and `execute_command`
(which reaches every one of GridFabrica's commands). The desktop app's control channel adds real
pointer and keyboard input, dialogs and screenshots. See [`docs/mcp.md`](docs/mcp.md),
[`docs/cli.md`](docs/cli.md) and [`docs/control-protocol.md`](docs/control-protocol.md).

## Architecture

GridFabrica is an engine-first Cargo workspace with enforced layering (`cargo xtask layers`). The
egui frontend is a separate crate, so the UI can be swapped without touching the engine.

| Layer | Crates |
|---|---|
| L0 | `core` (addresses, values, errors, dates, input parsing) · `numfmt` (number format codes) |
| L1 | `formula` (lexer, parser, printer, reference adjustment) · `functions` (worksheet function library) |
| L2 | `model` (workbook, sheets, copy-on-write cells, styles, tables, charts…) · `calc` (dependency graph, evaluator, dynamic arrays) · `xlsx` (XLSX and CSV) |
| L3 | `chart` (toolkit-free chart layout and rendering) |
| L5 | `engine` (session, commands, history, clipboard, fill, sort, filter, file I/O) |
| L6 | `ui-egui` (Excel-style UI, control channel) · `mcp` (MCP server) |
| L7 | `apps/gridfabrica`, `apps/gridfabrica-cli`, `apps/gridfabrica-web` |

- **Contributor and agent rules** (clean-room, asset policy, never-crash, quality gates): [`AGENTS.md`](AGENTS.md)
- **Bundled assets:** each one is listed with its licence in [`ATTRIBUTION.md`](ATTRIBUTION.md)
- **Releases:** signed macOS, Windows, Linux, BSD and web builds; see [`docs/releasing.md`](docs/releasing.md)

## Roadmap

GridFabrica is pre-alpha and moving fast. It covers 89% of Excel's ribbon and menu commands
([`docs/parity.md`](docs/parity.md)) and about 93% of its worksheet functions; weighted by depth,
we estimate about 65% of what Excel power users rely on. We're roughly 80% of the way to a first
alpha. Next up: signed release builds, XLSX fidelity on real-world files, performance on very large
sheets, then slicers, Solver, chart trendlines and a true Page Layout view. The plan and estimates
are in [ROADMAP.md](ROADMAP.md).

## Downloads

**Download GridFabrica** from GitHub: published builds are available in [releases](https://github.com/OpenFabrica/gridfabrica/releases). The table below lists the artifact names used by the release pipeline. `<ver>` in the file names is the version number, and `SHA256SUMS.txt` lists a checksum for every file.

### Windows

| Build | Installer | Portable |
|---|---|---|
| x64 (64-bit Intel/AMD) | `gridfabrica-<ver>-windows-x64.msi` | `gridfabrica-<ver>-windows-x64-portable.zip` |
| arm64 (Snapdragon and other ARM PCs) | `gridfabrica-<ver>-windows-arm64.msi` | `gridfabrica-<ver>-windows-arm64-portable.zip` |
| x86 (32-bit) | `gridfabrica-<ver>-windows-x86.msi` | `gridfabrica-<ver>-windows-x86-portable.zip` |

Installers and executables are code-signed.

### macOS

| Build | File | Notes |
|---|---|---|
| App, universal (Apple silicon + Intel) | `gridfabrica-<ver>-macos-universal.dmg` | Signed and notarized |
| Command-line tool, universal | `gridfabrica-cli-<ver>-macos-universal.zip` | Signed and notarized |

### Linux

| Format | x86_64 | aarch64 (ARM64) | Notes |
|---|---|---|---|
| AppImage | `gridfabrica-<ver>-linux-x86_64.AppImage` | `gridfabrica-<ver>-linux-aarch64.AppImage` | Runs anywhere; updates itself with [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) (`.zsync` files) |
| Flatpak | `gridfabrica-<ver>-linux-x86_64.flatpak` | `gridfabrica-<ver>-linux-aarch64.flatpak` | Sandboxed; `flatpak install --user <file>` |
| Debian/Ubuntu | `gridfabrica-<ver>-linux-x86_64.deb` | `gridfabrica-<ver>-linux-aarch64.deb` | |
| Fedora/RHEL/openSUSE | `gridfabrica-<ver>-linux-x86_64.rpm` | `gridfabrica-<ver>-linux-aarch64.rpm` | |
| Tarball | `gridfabrica-<ver>-linux-x86_64.tar.gz` | `gridfabrica-<ver>-linux-aarch64.tar.gz` | Unpack anywhere |

### FreeBSD

| Build | File |
|---|---|
| x86_64 | `gridfabrica-<ver>-freebsd-x86_64.tar.gz` |

### Web (WebAssembly)

| Build | File | Notes |
|---|---|---|
| Static site | `gridfabrica-web-<ver>.zip` | Runs in a modern browser; host it on any static server |

## License and credits

GridFabrica is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 ArtCraft Team and the GridCraft contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

All UI icons are drawn in code and are original; the sample workbooks and every screenshot in
this README originated in upstream GridCraft. Chinese fonts come from the optional shared
craft-fonts checkout; native builds embed them and web builds serve them independently. See
[localization](docs/localization.md).

GridFabrica is derived from GridCraft. Original author credits and copyright notices are retained.
ArtCraft trademark artwork is excluded from this fork. See [naming and migration](docs/naming.md).

<sub>Microsoft, Excel and Microsoft 365 are trademarks or registered trademarks of Microsoft Corporation in the United States and/or other countries. GridFabrica is an independent, open-source project and is not affiliated with, sponsored by or endorsed by Microsoft Corporation; these names are used only to describe the workflows it is compatible with.</sub>
