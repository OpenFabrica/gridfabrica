//! Design tokens and fonts.
//!
//! Colours are our own, tuned to read like a modern desktop spreadsheet. Fonts come from the
//! operating system at runtime, with an optional open Chinese font from craft-fonts: the UI uses the platform UI font, cells use the
//! best available match for the workbook font (Calibri/Carlito/Aptos/Arial…) with egui's built-in
//! fonts as the last resort.

use std::sync::Arc;

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, Visuals};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    pub window: Color32,
    pub ribbon: Color32,
    pub ribbon_border: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub accent: Color32,
    pub accent_dark: Color32,
    pub accent_soft: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub separator: Color32,
    pub grid_bg: Color32,
    pub gridline: Color32,
    pub header_bg: Color32,
    pub header_text: Color32,
    pub header_line: Color32,
    pub header_sel_bg: Color32,
    pub header_sel_text: Color32,
    pub header_all_bg: Color32,
    pub sel_fill: Color32,
    pub sel_border: Color32,
    pub input_bg: Color32,
    pub input_border: Color32,
    pub tab_bar: Color32,
    pub tab_active: Color32,
    pub status_bar: Color32,
    pub cell_text: Color32,
    pub menu_bg: Color32,
    pub shadow: Color32,
    pub danger: Color32,
}

impl Tokens {
    pub fn light() -> Tokens {
        Tokens {
            dark: false,
            window: Color32::from_rgb(0xF3, 0xF3, 0xF3),
            ribbon: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            ribbon_border: Color32::from_rgb(0xE3, 0xE3, 0xE3),
            text: Color32::from_rgb(0x24, 0x24, 0x24),
            text_dim: Color32::from_rgb(0x61, 0x61, 0x61),
            text_disabled: Color32::from_rgb(0xB0, 0xB0, 0xB0),
            accent: Color32::from_rgb(0x10, 0x7C, 0x41),
            accent_dark: Color32::from_rgb(0x0B, 0x5E, 0x31),
            accent_soft: Color32::from_rgb(0xDF, 0xF1, 0xE6),
            hover: Color32::from_rgb(0xEB, 0xEB, 0xEB),
            pressed: Color32::from_rgb(0xDD, 0xDD, 0xDD),
            separator: Color32::from_rgb(0xE0, 0xE0, 0xE0),
            grid_bg: Color32::WHITE,
            gridline: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            header_bg: Color32::from_rgb(0xF8, 0xF8, 0xF8),
            header_text: Color32::from_rgb(0x42, 0x42, 0x42),
            header_line: Color32::from_rgb(0xD6, 0xD6, 0xD6),
            header_sel_bg: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            header_sel_text: Color32::from_rgb(0x10, 0x7C, 0x41),
            header_all_bg: Color32::from_rgb(0xC8, 0xE6, 0xD3),
            sel_fill: Color32::from_rgba_unmultiplied(0x10, 0x7C, 0x41, 0x1C),
            sel_border: Color32::from_rgb(0x10, 0x7C, 0x41),
            input_bg: Color32::WHITE,
            input_border: Color32::from_rgb(0xD1, 0xD1, 0xD1),
            tab_bar: Color32::from_rgb(0xF3, 0xF3, 0xF3),
            tab_active: Color32::WHITE,
            status_bar: Color32::from_rgb(0xF3, 0xF3, 0xF3),
            cell_text: Color32::BLACK,
            menu_bg: Color32::WHITE,
            shadow: Color32::from_black_alpha(40),
            danger: Color32::from_rgb(0xC4, 0x2B, 0x1C),
        }
    }

    pub fn dark() -> Tokens {
        Tokens {
            dark: true,
            window: Color32::from_rgb(0x29, 0x29, 0x29),
            ribbon: Color32::from_rgb(0x33, 0x33, 0x33),
            ribbon_border: Color32::from_rgb(0x40, 0x40, 0x40),
            text: Color32::from_rgb(0xEE, 0xEE, 0xEE),
            text_dim: Color32::from_rgb(0xB0, 0xB0, 0xB0),
            text_disabled: Color32::from_rgb(0x70, 0x70, 0x70),
            accent: Color32::from_rgb(0x5C, 0xC5, 0x87),
            accent_dark: Color32::from_rgb(0x3F, 0xA8, 0x6C),
            accent_soft: Color32::from_rgb(0x23, 0x45, 0x31),
            hover: Color32::from_rgb(0x42, 0x42, 0x42),
            pressed: Color32::from_rgb(0x4D, 0x4D, 0x4D),
            separator: Color32::from_rgb(0x48, 0x48, 0x48),
            grid_bg: Color32::WHITE,
            gridline: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            header_bg: Color32::from_rgb(0x30, 0x30, 0x30),
            header_text: Color32::from_rgb(0xCC, 0xCC, 0xCC),
            header_line: Color32::from_rgb(0x45, 0x45, 0x45),
            header_sel_bg: Color32::from_rgb(0x45, 0x45, 0x45),
            header_sel_text: Color32::from_rgb(0x5C, 0xC5, 0x87),
            header_all_bg: Color32::from_rgb(0x2E, 0x55, 0x3E),
            sel_fill: Color32::from_rgba_unmultiplied(0x10, 0x7C, 0x41, 0x24),
            sel_border: Color32::from_rgb(0x10, 0x7C, 0x41),
            input_bg: Color32::from_rgb(0x1F, 0x1F, 0x1F),
            input_border: Color32::from_rgb(0x50, 0x50, 0x50),
            tab_bar: Color32::from_rgb(0x29, 0x29, 0x29),
            tab_active: Color32::from_rgb(0x3A, 0x3A, 0x3A),
            status_bar: Color32::from_rgb(0x29, 0x29, 0x29),
            cell_text: Color32::BLACK,
            menu_bg: Color32::from_rgb(0x30, 0x30, 0x30),
            shadow: Color32::from_black_alpha(90),
            danger: Color32::from_rgb(0xF1, 0x70, 0x5F),
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        if ctx.global_style().visuals.dark_mode { Tokens::dark() } else { Tokens::light() }
    }
}

pub const UI: &str = "ui";
pub const UI_BOLD: &str = "ui-bold";
pub const CELL: &str = "cell";
pub const CELL_BOLD: &str = "cell-bold";
pub const CELL_ITALIC: &str = "cell-italic";
pub const CELL_BOLD_ITALIC: &str = "cell-bold-italic";
pub const SERIF: &str = "serif";
pub const MONO: &str = "mono";
pub const CJK_SANS: &str = "cjk-sans";
pub const CJK_SERIF: &str = "cjk-serif";
pub const CJK_KAI: &str = "cjk-kai";
pub const CJK_FANG: &str = "cjk-fang";

#[derive(Clone, Default)]
struct RuntimeFonts(std::collections::BTreeMap<&'static str, Arc<FontData>>);

/// Install externally loaded Chinese fonts in this context only. Unknown IDs, invalid font
/// headers and files over 64 MiB return an error without changing the existing font set.
/// Repeated context setup preserves the runtime fonts; workbook styles remain unchanged.
pub fn set_runtime_fonts(ctx: &egui::Context, fonts: Vec<(&str, Vec<u8>)>) -> Result<(), String> {
    let mut runtime = RuntimeFonts::default();
    for (id, bytes) in fonts {
        let key = match id {
            "CJK_FONT" => "cjk-fallback",
            "CJK_SERIF_FONT" => "cjk-serif-fallback",
            "CJK_KAI_FONT" => "cjk-kai-fallback",
            "CJK_FANG_FONT" => "cjk-fang-fallback",
            _ => return Err(format!("Unknown font resource: {id}")),
        };
        if bytes.len() > 64 * 1024 * 1024 {
            return Err(format!("Font resource exceeds 64 MiB: {id}"));
        }
        read_fonts::FontRef::new(&bytes).map_err(|_| format!("Invalid font resource: {id}"))?;
        if runtime.0.insert(key, Arc::new(FontData::from_owned(bytes))).is_some() {
            return Err(format!("Duplicate font resource: {id}"));
        }
    }
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("gridfabrica.runtimeFonts"), runtime));
    install_fonts(ctx);
    ctx.request_repaint();
    Ok(())
}

/// Chinese faces actually available in this context. Their names are workbook font values,
/// independent of the localized labels used by selectors.
pub fn chinese_fonts(ctx: &egui::Context) -> Vec<&'static str> {
    ctx.data(|data| data.get_temp(egui::Id::new("gridfabrica.chineseFonts"))).unwrap_or_default()
}

pub fn install_fonts(ctx: &egui::Context) {
    let runtime: RuntimeFonts = ctx.data(|data| data.get_temp(egui::Id::new("gridfabrica.runtimeFonts"))).unwrap_or_default();
    let fonts = definitions(&runtime);
    let mut available = Vec::new();
    if CJK_FONT.is_some() || runtime.0.contains_key("cjk-fallback") {
        available.push("Noto Sans CJK SC");
    }
    // Common workbook names are compatibility choices. Their actual open substitute is shown
    // in a tooltip; storing the requested name preserves interoperability with other readers.
    for (key, names) in [
        ("cjk-serif-fallback", &["宋体"][..]),
        ("cjk-fallback", &["黑体", "微软雅黑", "等线"][..]),
        ("cjk-kai-fallback", &["楷体"][..]),
        ("cjk-fang-fallback", &["仿宋"][..]),
    ] {
        if fonts.font_data.contains_key(key) {
            available.extend_from_slice(names);
        }
    }
    if CJK_SERIF_FONT.is_some() || runtime.0.contains_key("cjk-serif-fallback") {
        available.push("Noto Serif CJK SC");
    }
    if CJK_KAI_FONT.is_some() || runtime.0.contains_key("cjk-kai-fallback") {
        available.push("LXGW WenKai");
    }
    if CJK_FANG_FONT.is_some() || runtime.0.contains_key("cjk-fang-fallback") {
        available.push("Zhuque Fangsong (technical preview)");
    }
    if fonts.font_data.contains_key("cjk-serif-system") {
        available.push("Source Han Serif CN Light");
    }
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("gridfabrica.chineseFonts"), available));
    ctx.set_fonts(fonts);
}

/// Open font used to display a common workbook font name. This does not rename stored styles
/// or claim that a proprietary font is bundled; exact glyphs and metrics may differ.
pub fn font_substitute(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "宋体" | "simsun" | "nsimsun" => Some("Noto Serif CJK SC"),
        "黑体" | "simhei" | "微软雅黑" | "microsoft yahei" | "等线" | "dengxian" => Some("Noto Sans CJK SC"),
        "楷体" | "kaiti" | "楷体_gb2312" => Some("LXGW WenKai"),
        "仿宋" | "fangsong" | "仿宋_gb2312" => Some("Zhuque Fangsong (technical preview)"),
        _ => None,
    }
}

pub fn ui_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(UI.into()))
}
pub fn ui_bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(UI_BOLD.into()))
}

/// The font family to use for a cell font name and weight.
pub fn cell_family(name: &str, bold: bool, italic: bool) -> FontFamily {
    if let Some(substitute) = font_substitute(name) {
        return cell_family(substitute, bold, italic);
    }
    let lower = name.to_ascii_lowercase();
    if lower == "lxgw wenkai" {
        return FontFamily::Name(CJK_KAI.into());
    }
    if lower.starts_with("zhuque fangsong") {
        return FontFamily::Name(CJK_FANG.into());
    }
    if lower == "source han serif cn light" {
        return FontFamily::Name("cjk-serif-system".into());
    }
    if lower.contains("noto sans cjk") || lower.contains("source han sans") || name == "思源黑体" {
        return FontFamily::Name(CJK_SANS.into());
    }
    if lower.contains("source han serif") || lower.contains("noto serif cjk") || name == "思源宋体" {
        return FontFamily::Name(CJK_SERIF.into());
    }
    if lower.contains("courier") || lower.contains("mono") || lower.contains("consolas") {
        return FontFamily::Name(MONO.into());
    }
    if lower.contains("times")
        || lower.contains("georgia")
        || lower.contains("cambria")
        || lower.contains("serif") && !lower.contains("sans")
        || lower.contains("garamond")
    {
        return FontFamily::Name(SERIF.into());
    }
    FontFamily::Name(
        match (bold, italic) {
            (true, true) => CELL_BOLD_ITALIC,
            (true, false) => CELL_BOLD,
            (false, true) => CELL_ITALIC,
            (false, false) => CELL,
        }
        .into(),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn try_load(paths: &[(&str, u32)]) -> Option<FontData> {
    for (p, index) in paths {
        if let Ok(bytes) = std::fs::read(p) {
            let mut fd = FontData::from_owned(bytes);
            fd.index = *index;
            return Some(fd);
        }
    }
    None
}

#[cfg(target_arch = "wasm32")]
fn try_load(_paths: &[(&str, u32)]) -> Option<FontData> {
    None
}

fn home_fonts(file: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    format!("{home}/Library/Fonts/{file}")
}

/// Candidate font files per role, in preference order.
fn candidates(role: &str) -> Vec<(String, u32)> {
    let mac_sys = "/System/Library/Fonts";
    let mac_sup = "/System/Library/Fonts/Supplemental";
    let win = std::env::var("WINDIR").map(|w| format!("{w}\\Fonts")).unwrap_or_else(|_| "C:\\Windows\\Fonts".into());
    let lin = ["/usr/share/fonts/truetype", "/usr/share/fonts", "/usr/local/share/fonts", "/usr/share/fonts/TTF"];
    let mut v: Vec<(String, u32)> = Vec::new();
    let mut add = |s: String, i: u32| v.push((s, i));
    match role {
        "ui" => {
            add(format!("{mac_sys}/SFNS.ttf"), 0);
            add(format!("{mac_sys}/HelveticaNeue.ttc"), 0);
            add(format!("{win}\\segoeui.ttf"), 0);
            for d in lin {
                add(format!("{d}/cantarell/Cantarell-Regular.otf"), 0);
                add(format!("{d}/noto/NotoSans-Regular.ttf"), 0);
                add(format!("{d}/dejavu/DejaVuSans.ttf"), 0);
                add(format!("{d}/liberation/LiberationSans-Regular.ttf"), 0);
            }
        }
        "ui-bold" => {
            add(format!("{mac_sys}/HelveticaNeue.ttc"), 1);
            add(format!("{win}\\segoeuib.ttf"), 0);
            for d in lin {
                add(format!("{d}/noto/NotoSans-Bold.ttf"), 0);
                add(format!("{d}/dejavu/DejaVuSans-Bold.ttf"), 0);
                add(format!("{d}/liberation/LiberationSans-Bold.ttf"), 0);
            }
        }
        "cell" | "cell-bold" | "cell-italic" | "cell-bold-italic" => {
            let (carlito, calibri_win, calibri_mac, arial_mac, arial_win, lib, dejavu) = match role {
                "cell" => {
                    ("Carlito-Regular.ttf", "calibri.ttf", "Calibri.ttf", "Arial.ttf", "arial.ttf", "LiberationSans-Regular.ttf", "DejaVuSans.ttf")
                }
                "cell-bold" => (
                    "Carlito-Bold.ttf",
                    "calibrib.ttf",
                    "Calibri Bold.ttf",
                    "Arial Bold.ttf",
                    "arialbd.ttf",
                    "LiberationSans-Bold.ttf",
                    "DejaVuSans-Bold.ttf",
                ),
                "cell-italic" => (
                    "Carlito-Italic.ttf",
                    "calibrii.ttf",
                    "Calibri Italic.ttf",
                    "Arial Italic.ttf",
                    "ariali.ttf",
                    "LiberationSans-Italic.ttf",
                    "DejaVuSans-Oblique.ttf",
                ),
                _ => (
                    "Carlito-BoldItalic.ttf",
                    "calibriz.ttf",
                    "Calibri Bold Italic.ttf",
                    "Arial Bold Italic.ttf",
                    "arialbi.ttf",
                    "LiberationSans-BoldItalic.ttf",
                    "DejaVuSans-BoldOblique.ttf",
                ),
            };
            // Calibri if the user has it installed system-wide (never from an Office bundle).
            add(format!("/Library/Fonts/{calibri_mac}"), 0);
            add(home_fonts(calibri_mac), 0);
            add(format!("{win}\\{calibri_win}"), 0);
            for d in lin {
                add(format!("{d}/crosextra/{carlito}"), 0);
                add(format!("{d}/carlito/{carlito}"), 0);
            }
            add(home_fonts(carlito), 0);
            add(format!("{mac_sup}/{arial_mac}"), 0);
            add(format!("{win}\\{arial_win}"), 0);
            for d in lin {
                add(format!("{d}/liberation/{lib}"), 0);
                add(format!("{d}/dejavu/{dejavu}"), 0);
            }
        }
        "serif" => {
            add(format!("{mac_sup}/Times New Roman.ttf"), 0);
            add(format!("{win}\\times.ttf"), 0);
            for d in lin {
                add(format!("{d}/liberation/LiberationSerif-Regular.ttf"), 0);
                add(format!("{d}/dejavu/DejaVuSerif.ttf"), 0);
            }
        }
        "mono" => {
            add(format!("{mac_sys}/SFNSMono.ttf"), 0);
            add(format!("{mac_sys}/Menlo.ttc"), 0);
            add(format!("{win}\\consola.ttf"), 0);
            for d in lin {
                add(format!("{d}/dejavu/DejaVuSansMono.ttf"), 0);
            }
        }
        _ => {}
    }
    v
}

include!(concat!(env!("OUT_DIR"), "/cjk_font.rs"));

/// Chinese fallback: the optional OFL font is available on every target; native builds also
/// discover installed CJK fonts. Font assets stay outside this repository.
fn cjk_font() -> Option<FontData> {
    if let Some(bytes) = CJK_FONT {
        return Some(FontData::from_static(bytes));
    }
    let home = home_fonts("SourceHanSansCN-Regular.otf");
    let windows = std::env::var("WINDIR").unwrap_or_else(|_| r"C:\Windows".into());
    let windows = std::path::PathBuf::from(windows).join("Fonts/msyh.ttc");
    let windows = windows.to_string_lossy();
    try_load(&[
        (&home, 0),
        ("/System/Library/Fonts/PingFang.ttc", 0),
        ("/System/Library/Fonts/STHeiti Light.ttc", 0),
        ("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc", 2),
        ("/usr/share/fonts/truetype/wqy/wqy-microhei.ttc", 0),
        (&windows, 0),
    ])
}

/// Builds the font set from system fonts, optional craft-fonts, and egui defaults.
pub fn font_definitions() -> FontDefinitions {
    definitions(&RuntimeFonts::default())
}

fn definitions(runtime: &RuntimeFonts) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let base_prop: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let base_mono: Vec<String> = fonts.families.get(&FontFamily::Monospace).cloned().unwrap_or_default();
    for role in [UI, UI_BOLD, CELL, CELL_BOLD, CELL_ITALIC, CELL_BOLD_ITALIC, SERIF, MONO] {
        let paths = candidates(role);
        let refs: Vec<(&str, u32)> = paths.iter().map(|(p, i)| (p.as_str(), *i)).collect();
        let mut chain: Vec<String> = Vec::new();
        if let Some(fd) = try_load(&refs) {
            let key = format!("sys-{role}");
            fonts.font_data.insert(key.clone(), Arc::new(fd));
            chain.push(key);
        } else if role == UI_BOLD {
            chain.push(format!("sys-{UI}"));
        } else if role.starts_with("cell-") {
            chain.push(format!("sys-{CELL}"));
        }
        chain.extend(if role == MONO { base_mono.clone() } else { base_prop.clone() });
        chain.retain(|k| fonts.font_data.contains_key(k));
        fonts.families.insert(FontFamily::Name(role.into()), chain);
    }
    // Explicit Chinese faces put the CJK font first (including Latin glyphs). The ordinary
    // workbook families retain their existing primary face and use CJK only for missing glyphs.
    let sans_key = "cjk-fallback".to_string();
    let serif_key = "cjk-serif-fallback".to_string();
    let sans = runtime.0.get("cjk-fallback").or_else(|| runtime.0.values().next()).cloned().or_else(|| cjk_font().map(Arc::new));
    if let Some(font) = sans {
        fonts.font_data.insert(sans_key.clone(), font);
        for chain in fonts.families.values_mut() {
            chain.push(sans_key.clone());
        }
    }
    for (key, bytes) in [(serif_key.as_str(), CJK_SERIF_FONT), ("cjk-kai-fallback", CJK_KAI_FONT), ("cjk-fang-fallback", CJK_FANG_FONT)] {
        if let Some(bytes) = bytes {
            fonts.font_data.insert(key.to_string(), Arc::new(FontData::from_static(bytes)));
        }
    }
    if let Some(font) = try_load(&[(&home_fonts("SourceHanSerifCN-Light.otf"), 0)]) {
        fonts.font_data.insert("cjk-serif-system".into(), Arc::new(font));
    }
    for (key, font) in &runtime.0 {
        fonts.font_data.insert((*key).into(), font.clone());
    }
    for (role, primary) in [
        (CJK_SANS, sans_key.as_str()),
        (CJK_SERIF, serif_key.as_str()),
        (CJK_KAI, "cjk-kai-fallback"),
        (CJK_FANG, "cjk-fang-fallback"),
        ("cjk-serif-system", "cjk-serif-system"),
    ] {
        let mut chain = vec![primary.to_string(), "cjk-serif-system".into(), sans_key.clone()];
        chain.extend(base_prop.clone());
        chain.retain(|key| fonts.font_data.contains_key(key));
        chain.dedup();
        fonts.families.insert(FontFamily::Name(role.into()), chain);
    }
    // Default proportional text in widgets uses the UI font.
    if let Some(ui) = fonts.families.get(&FontFamily::Name(UI.into())).cloned() {
        fonts.families.insert(FontFamily::Proportional, ui);
    }
    fonts
}

pub fn apply(ctx: &egui::Context, dark: bool) {
    let t = if dark { Tokens::dark() } else { Tokens::light() };
    let mut v = if dark { Visuals::dark() } else { Visuals::light() };
    v.panel_fill = t.window;
    v.window_fill = t.menu_bg;
    v.extreme_bg_color = t.input_bg;
    v.selection.bg_fill = t.accent_soft;
    v.selection.stroke = Stroke::new(1.0, t.accent);
    v.hyperlink_color = t.accent;
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.input_border);
    v.window_corner_radius = egui::CornerRadius::same(10);
    v.menu_corner_radius = egui::CornerRadius::same(8);
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: t.shadow };
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, ui_font(13.0));
        s.text_styles.insert(egui::TextStyle::Button, ui_font(13.0));
        s.text_styles.insert(egui::TextStyle::Small, ui_font(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, ui_bold(18.0));
        s.text_styles.insert(egui::TextStyle::Monospace, FontId::new(12.0, FontFamily::Name(MONO.into())));
    });
}

/// Model colour → egui colour.
pub fn color32(rgb: [u8; 3]) -> Color32 {
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

#[cfg(test)]
mod font_tests {
    use super::*;

    #[test]
    fn runtime_fonts_survive_setup_and_reject_invalid_updates_atomically() {
        let ctx = egui::Context::default();
        let other = egui::Context::default();
        let definitions = font_definitions();
        let Some(font) = definitions.font_data.get("cjk-fallback") else { return };
        set_runtime_fonts(&ctx, vec![("CJK_FONT", font.font.to_vec())]).unwrap();
        install_fonts(&ctx);
        install_fonts(&ctx);
        assert!(chinese_fonts(&ctx).contains(&"Noto Sans CJK SC"));
        for update in [vec![("unknown", Vec::new())], vec![("CJK_KAI_FONT", b"<html>404</html>".to_vec())]] {
            assert!(set_runtime_fonts(&ctx, update).is_err());
            let saved: RuntimeFonts = ctx.data(|data| data.get_temp(egui::Id::new("gridfabrica.runtimeFonts"))).unwrap();
            assert_eq!(saved.0.len(), 1);
            assert_eq!(saved.0.get("cjk-fallback").unwrap().font.as_ref(), font.font.as_ref());
        }
        assert!(other.data(|data| data.get_temp::<RuntimeFonts>(egui::Id::new("gridfabrica.runtimeFonts"))).is_none());
    }
}
