//! Validate and compile the translation catalog and optional Chinese fallback font.
use std::{collections::BTreeMap, fmt::Write as _, path::PathBuf};

pub fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let out = PathBuf::from(std::env::var("OUT_DIR")?);
    let catalog = PathBuf::from("src/i18n/zh-CN.json");
    println!("cargo::rerun-if-changed={}", catalog.display());
    let entries: BTreeMap<String, String> = serde_json::from_slice(&std::fs::read(catalog)?)?;
    let mut source = String::from("fn chinese(text: &str) -> Option<&'static str> { match text {\n");
    for (key, value) in entries {
        if key.trim().is_empty() || value.trim().is_empty() {
            return Err("empty translation key or value".into());
        }
        writeln!(source, "{key:?} => Some({value:?}),")?;
    }
    source.push_str("_ => None, }}\n");
    std::fs::write(out.join("translations.rs"), source)?;

    println!("cargo::rerun-if-env-changed=CRAFT_FONTS_DIR");
    println!("cargo::rerun-if-env-changed=CRAFT_FONTS_REQUIRED");
    let font_file = "fonts/noto-sans-cjk-sc/NotoSansCJKsc-Regular.otf";
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("../..");
    // A developer checkout is optional too. Prefer an explicit path, then a checkout in the
    // repo's ignored craft-fonts directory or beside the app. Never fetch fonts during build.
    let directory = std::env::var_os("CRAFT_FONTS_DIR")
        .map(PathBuf::from)
        .or_else(|| [root.join("craft-fonts"), root.join("../craft-fonts")].into_iter().find(|dir| dir.join(font_file).is_file()));
    let mut source = String::new();
    let wasm = std::env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|arch| arch == "wasm32");
    let mut assets = String::from("pub const WEB_FONT_ASSETS: &[(&str, &str, u64)] = &[\n");
    for (constant, file) in [
        ("CJK_FONT", font_file),
        ("CJK_SERIF_FONT", "fonts/noto-serif-cjk-sc/NotoSerifCJKsc-Regular.otf"),
        ("CJK_KAI_FONT", "fonts/lxgw-wenkai/LXGWWenKai-Regular.ttf"),
        ("CJK_FANG_FONT", "fonts/zhuque-fangsong/ZhuqueFangsong-Regular.ttf"),
    ] {
        let font = directory
            .as_ref()
            .map(|dir| {
                let path = dir.join(file);
                println!("cargo::rerun-if-changed={}", path.display());
                path.canonicalize()
            })
            .transpose();
        let value = match font {
            Ok(Some(path)) if wasm => {
                let directory = path.parent().and_then(|p| p.parent()).and_then(|p| p.parent()).ok_or("invalid font path")?;
                let manifest_path = directory.join("fonts/manifest.txt");
                println!("cargo::rerun-if-changed={}", manifest_path.display());
                let manifest = std::fs::read_to_string(manifest_path)?;
                let hash = manifest
                    .lines()
                    .find_map(|line| {
                        let fields: Vec<_> = line.split(" | ").collect();
                        (fields.get(2) == Some(&file)).then(|| fields.get(6).copied()).flatten()
                    })
                    .ok_or_else(|| format!("font absent from manifest: {file}"))?;
                if hash.len() != 64 || !hash.bytes().all(|c| c.is_ascii_hexdigit()) {
                    return Err(format!("invalid font checksum: {file}").into());
                }
                let name = path.file_stem().and_then(|name| name.to_str()).ok_or("invalid font name")?;
                let extension = path.extension().and_then(|name| name.to_str()).ok_or("invalid font extension")?;
                let url = format!("fonts/{name}-{hash}.{extension}");
                writeln!(assets, "({constant:?}, {url:?}, {}),", path.metadata()?.len())?;
                "None".into()
            }
            Ok(Some(path)) => format!("Some(include_bytes!({:?}))", path.to_string_lossy()),
            Ok(None) => {
                if std::env::var_os("CRAFT_FONTS_REQUIRED").is_some() {
                    return Err("CRAFT_FONTS_REQUIRED needs CRAFT_FONTS_DIR".into());
                }
                "None".into()
            }
            Err(e) => {
                if std::env::var_os("CRAFT_FONTS_REQUIRED").is_some() {
                    return Err(e.into());
                }
                println!("cargo::warning=Chinese fallback font unavailable: {e}");
                "None".into()
            }
        };
        writeln!(source, "const {constant}: Option<&[u8]> = {value};")?;
    }
    assets.push_str("];\n");
    source.push_str(&assets);
    std::fs::write(out.join("cjk_font.rs"), source)?;
    Ok(())
}
