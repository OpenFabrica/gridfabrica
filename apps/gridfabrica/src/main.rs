//! gridfabrica desktop app.
//!
//! Usage: `gridfabrica [--control <port>] [--sample <name>] [files…]`
//!
//! `--control <port>` (or `GRIDFABRICA_CONTROL_PORT`) starts a localhost JSON-lines control
//! server: `{"id":1,"method":"ui.inspect","params":{}}` → `{"id":1,"ok":true,"result":…}`.
//! See `gridfabrica_ui_egui::control` and `docs/control-protocol.md` for the methods.
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod control_server;
#[cfg(target_os = "macos")]
mod native_menu;

use gridfabrica_engine::Session;
use gridfabrica_ui_egui::{Services, SheetApp};
use serde_json::json;

struct App(SheetApp, #[cfg(target_os = "macos")] Option<native_menu::NativeMenu>);

impl eframe::App for App {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "macos")]
        {
            if self.1.as_ref().is_none_or(|menu| menu.language != self.0.ui.language)
                && std::env::var_os("GRIDFABRICA_NO_NATIVE_MENU").is_none()
                && std::env::var_os("GRIDCRAFT_NO_NATIVE_MENU").is_none()
            {
                self.1 = Some(native_menu::NativeMenu::install(ctx, self.0.ui.language));
            }
            if let Some(m) = &self.1 {
                m.poll(&mut self.0, ctx);
            }
        }
        self.0.logic(ctx);
    }
    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.0.raw_input_hook(raw);
        // Files dropped on the window open as workbooks.
        for f in std::mem::take(&mut raw.dropped_files) {
            if let Some(p) = f.path().to_str().map(str::to_string).filter(|p| !p.is_empty()) {
                self.0.open_path(&p);
            }
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.0.ui(ui);
    }
    fn on_exit(&mut self) {
        save_prefs(&self.0);
    }
}

fn config_dir_for(name: &str) -> Option<std::path::PathBuf> {
    if cfg!(target_os = "macos") {
        std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join("Library/Application Support").join(name))
    } else if cfg!(windows) {
        std::env::var_os("APPDATA").map(|a| std::path::PathBuf::from(a).join(name))
    } else {
        std::env::var_os("XDG_CONFIG_HOME")
            .map(std::path::PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| std::path::PathBuf::from(h).join(".config")))
            .map(|c| c.join(name))
    }
}

fn config_dir() -> Option<std::path::PathBuf> {
    config_dir_for(if cfg!(any(target_os = "macos", windows)) { "GridFabrica" } else { "gridfabrica" })
}

fn read_preference(dir: &std::path::Path, legacy: Option<&std::path::Path>, file: &str) -> Option<Vec<u8>> {
    // Only a missing new file reads the legacy file. A damaged or inaccessible new preference
    // is not silently replaced with older settings, and old files are never modified.
    match std::fs::read(dir.join(file)) {
        Ok(bytes) => Some(bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => legacy.and_then(|dir| std::fs::read(dir.join(file)).ok()),
        Err(_) => None,
    }
}

fn load_prefs(app: &mut SheetApp) {
    if std::env::var_os("GRIDFABRICA_NO_PREFS").is_some() || std::env::var_os("GRIDCRAFT_NO_PREFS").is_some() {
        return;
    }
    let Some(dir) = config_dir() else { return };
    let legacy = config_dir_for(if cfg!(any(target_os = "macos", windows)) { "GridCraft" } else { "gridcraft" });
    if let Some(b) = read_preference(&dir, legacy.as_deref(), "ui.json")
        && let Ok(ui) = serde_json::from_slice(&b)
    {
        app.ui = ui;
    }
    if let Some(b) = read_preference(&dir, legacy.as_deref(), "prefs.json")
        && let Ok(p) = serde_json::from_slice(&b)
    {
        app.session.prefs = p;
    }
}

fn save_prefs(app: &SheetApp) {
    if std::env::var_os("GRIDFABRICA_NO_PREFS").is_some() || std::env::var_os("GRIDCRAFT_NO_PREFS").is_some() {
        return;
    }
    let Some(dir) = config_dir() else { return };
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(b) = serde_json::to_vec_pretty(&app.ui) {
        let _ = std::fs::write(dir.join("ui.json"), b);
    }
    if let Ok(b) = serde_json::to_vec_pretty(&app.session.prefs) {
        let _ = std::fs::write(dir.join("prefs.json"), b);
    }
}

fn services() -> Services {
    Services {
        pick_open: Some(Box::new(|| {
            rfd::FileDialog::new()
                .add_filter("Spreadsheets", &["xlsx", "xlsm", "csv", "tsv", "txt", "json"])
                .add_filter("Excel Workbook", &["xlsx", "xlsm"])
                .add_filter("CSV", &["csv"])
                .pick_file()
                .and_then(|p| p.to_str().map(str::to_string))
        })),
        pick_save: Some(Box::new(|suggested: &str| {
            let stem = std::path::Path::new(suggested).file_stem().and_then(|s| s.to_str()).unwrap_or("Book1").to_string();
            rfd::FileDialog::new()
                .set_file_name(format!("{stem}.xlsx"))
                .add_filter("Excel Workbook (.xlsx)", &["xlsx"])
                .add_filter("CSV UTF-8 (.csv)", &["csv"])
                .add_filter("Web Page (.html)", &["html"])
                .save_file()
                .and_then(|p| p.to_str().map(str::to_string))
        })),
        open_url: Some(Box::new(|url: &str| {
            let cmd = if cfg!(target_os = "macos") {
                "open"
            } else if cfg!(windows) {
                "explorer"
            } else {
                "xdg-open"
            };
            let _ = std::process::Command::new(cmd).arg(url).spawn();
        })),
        ..Default::default()
    }
}

fn icon() -> Option<egui::IconData> {
    let bytes = include_bytes!("../../../assets/app-icon/hicolor/256x256/apps/ai.storyteller.gridfabrica.png");
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    Some(egui::IconData { width: img.width(), height: img.height(), rgba: img.into_raw() })
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("GridFabrica {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    let mut control_port: Option<u16> =
        std::env::var("GRIDFABRICA_CONTROL_PORT").or_else(|_| std::env::var("GRIDCRAFT_CONTROL_PORT")).ok().and_then(|p| p.parse().ok());
    let mut sample: Option<String> = None;
    let mut files: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--control" => {
                i += 1;
                control_port = args.get(i).and_then(|p| p.parse().ok());
            }
            "--sample" => {
                i += 1;
                sample = Some(args.get(i).cloned().unwrap_or_else(|| "sales".into()));
            }
            a if a.starts_with("-psn_") => {}
            a => files.push(a.to_string()),
        }
        i += 1;
    }
    let mut session = Session::new();
    if let Some(s) = &sample
        && let Err(e) = session.execute("file.new", json!({"sample": s}))
    {
        eprintln!("sample: {e}");
    }
    for f in &files {
        if let Err(e) = session.execute("file.open", json!({"path": f})) {
            eprintln!("{f}: {e}");
        }
    }
    if session.documents().is_empty() {
        session.new_workbook();
    }
    let mut app = SheetApp::new(session, services());
    let control_port = control_port;
    load_prefs(&mut app);
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("GridFabrica")
        .with_inner_size([1440.0, 900.0])
        .with_min_inner_size([640.0, 420.0])
        .with_app_id("ai.storyteller.gridfabrica")
        .with_drag_and_drop(true);
    if cfg!(target_os = "macos") {
        viewport = viewport.with_fullsize_content_view(true).with_titlebar_shown(false).with_title_shown(false);
    }
    if let Some(i) = icon() {
        viewport = viewport.with_icon(i);
    }
    let options = eframe::NativeOptions { viewport, ..Default::default() };
    eframe::run_native(
        "GridFabrica",
        options,
        Box::new(move |cc| {
            SheetApp::setup_context(&cc.egui_ctx, app.ui.dark);
            if let Some(port) = control_port {
                match control_server::start(port, cc.egui_ctx.clone()) {
                    Ok(rx) => app.control_rx = Some(rx),
                    Err(e) => eprintln!("control channel on port {port}: {e}"),
                }
            }
            Ok(Box::new(App(
                app,
                #[cfg(target_os = "macos")]
                None,
            )))
        }),
    )
}

#[cfg(test)]
mod preference_tests {
    use super::read_preference;

    #[test]
    fn legacy_preferences_are_read_only_when_the_new_file_is_missing() -> Result<(), Box<dyn std::error::Error>> {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos();
        let root = std::env::temp_dir().join(format!("gridfabrica-preferences-{}-{stamp}", std::process::id()));
        std::fs::create_dir(&root)?;
        let new = root.join("gridfabrica");
        let old = root.join("GridCraft");
        std::fs::create_dir(&new)?;
        std::fs::create_dir(&old)?;
        std::fs::write(old.join("ui.json"), b"old preference")?;
        assert_eq!(read_preference(&new, Some(&old), "ui.json"), Some(b"old preference".to_vec()));
        std::fs::write(new.join("ui.json"), b"new preference")?;
        assert_eq!(read_preference(&new, Some(&old), "ui.json"), Some(b"new preference".to_vec()));
        std::fs::remove_file(new.join("ui.json"))?;
        std::fs::create_dir(new.join("ui.json"))?;
        assert_eq!(read_preference(&new, Some(&old), "ui.json"), None);
        assert_eq!(std::fs::read(old.join("ui.json"))?, b"old preference");
        std::fs::remove_dir_all(root)?;
        Ok(())
    }
}
