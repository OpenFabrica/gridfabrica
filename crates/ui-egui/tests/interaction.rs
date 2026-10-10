//! UI interaction tests: real egui input through the whole app (no GPU needed).

use egui::{Event, Key, Modifiers};
use gridfabrica_engine::Session;
use gridfabrica_engine::core::{CellRef, Value};
use gridfabrica_ui_egui::SheetApp;
use serde_json::json;

fn harness(session: Session) -> egui_kittest::Harness<'static, SheetApp> {
    let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
        |ui, app: &mut SheetApp| {
            let ctx = ui.ctx().clone();
            app.logic(&ctx);
            app.ui(ui);
        },
        SheetApp::new(session, Default::default()),
    );
    h.run_steps(4);
    h
}

fn blank() -> Session {
    let mut s = Session::new();
    s.new_workbook();
    s
}

fn key(h: &mut egui_kittest::Harness<'static, SheetApp>, k: Key, m: Modifiers) {
    h.input_mut().events.push(Event::Key { key: k, physical_key: None, pressed: true, repeat: false, modifiers: m });
    h.input_mut().events.push(Event::Key { key: k, physical_key: None, pressed: false, repeat: false, modifiers: m });
    h.run_steps(2);
}

fn text(h: &mut egui_kittest::Harness<'static, SheetApp>, t: &str) {
    h.input_mut().events.push(Event::Text(t.into()));
    h.run_steps(2);
}

fn value(h: &egui_kittest::Harness<'static, SheetApp>, a: &str) -> Value {
    h.state().session.active().and_then(|d| d.wb.active().map(|s| s.value(CellRef::parse(a).unwrap_or_default()))).unwrap_or_default()
}

#[test]
fn typing_enters_values_and_moves_down() {
    let mut h = harness(blank());
    text(&mut h, "42");
    assert!(h.state().editor.is_some(), "typing starts editing");
    key(&mut h, Key::Enter, Modifiers::NONE);
    assert_eq!(value(&h, "A1"), Value::Number(42.0));
    text(&mut h, "=A1*2");
    key(&mut h, Key::Tab, Modifiers::NONE);
    assert_eq!(value(&h, "A2"), Value::Number(84.0));
    let active = h.state().session.active().map(|d| d.selection.active.a1());
    assert_eq!(active.as_deref(), Some("B2"));
}

#[test]
fn arrows_escape_and_undo() {
    let mut h = harness(blank());
    key(&mut h, Key::ArrowDown, Modifiers::NONE);
    key(&mut h, Key::ArrowRight, Modifiers::NONE);
    assert_eq!(h.state().session.active().map(|d| d.selection.active.a1()).as_deref(), Some("B2"));
    text(&mut h, "hello");
    key(&mut h, Key::Escape, Modifiers::NONE);
    assert_eq!(value(&h, "B2"), Value::Empty);
    text(&mut h, "x");
    key(&mut h, Key::Enter, Modifiers::NONE);
    assert_eq!(value(&h, "B2"), Value::from("x"));
    key(&mut h, Key::Z, Modifiers::COMMAND);
    assert_eq!(value(&h, "B2"), Value::Empty);
}

#[test]
fn shortcuts_format_and_extend() {
    let mut s = blank();
    s.execute("range.setValues", json!({"range": "A1", "values": [[1, 2], [3, 4]]})).unwrap();
    let mut h = harness(s);
    key(&mut h, Key::ArrowRight, Modifiers::SHIFT);
    key(&mut h, Key::ArrowDown, Modifiers::SHIFT);
    assert_eq!(h.state().session.active().map(|d| d.selection.a1()).as_deref(), Some("A1:B2"));
    key(&mut h, Key::B, Modifiers::COMMAND);
    let bold = h.state_mut().session.run("cell.get", json!({"cell": "B2"})).unwrap()["style"]["font"]["bold"].clone();
    assert_eq!(bold, json!(true));
    key(&mut h, Key::Delete, Modifiers::NONE);
    assert_eq!(value(&h, "A1"), Value::Empty);
}

#[test]
fn renders_every_ribbon_tab_without_panicking() {
    let mut s = Session::new();
    s.execute("file.new", json!({"sample": "sales"})).unwrap();
    let mut h = harness(s);
    for tab in ["Home", "Insert", "Draw", "Page Layout", "Formulas", "Data", "Review", "View", "Automate"] {
        h.state_mut().ui.ribbon_tab = tab.into();
        h.run_steps(2);
    }
    h.state_mut().ui.dark = true;
    h.run_steps(3);
}

#[test]
fn dialogs_open_and_close() {
    let mut h = harness(blank());
    for name in [
        "formatCells",
        "insertFunction",
        "find",
        "sort",
        "nameManager",
        "dataValidation",
        "pasteSpecial",
        "commandSearch",
        "spelling",
        "goTo",
        "about",
    ] {
        h.state_mut().open_dialog(name, json!({}));
        h.run_steps(2);
        key(&mut h, Key::Escape, Modifiers::NONE);
        h.state_mut().dialog = None;
        h.state_mut().message = None;
    }
}

#[test]
fn about_renders_every_tab() {
    let mut h = harness(blank());
    for tab in ["About", "Contributors", "Models"] {
        h.state_mut().open_dialog("about", json!({"tab": tab}));
        h.run_steps(2);
        assert_eq!(h.state().dialog.as_ref().map(|d| d.tab.clone()).as_deref(), Some(tab));
        h.state_mut().dialog = None;
    }
}

#[test]
fn column_autocomplete_completes_on_enter() {
    let mut s = blank();
    s.execute("range.setValues", json!({"range": "A1", "values": [["North"], ["East"], ["Eastern"]]})).unwrap();
    s.execute("selection.set", json!({"cell": "A4"})).unwrap();
    let mut h = harness(s);
    text(&mut h, "No");
    key(&mut h, Key::Enter, Modifiers::NONE);
    assert_eq!(value(&h, "A4"), Value::from("North"));
    text(&mut h, "Ea"); // ambiguous: East / Eastern
    key(&mut h, Key::Enter, Modifiers::NONE);
    assert_eq!(value(&h, "A5"), Value::from("Ea"));
}

#[test]
fn language_switch_is_a_command_and_preserves_workbook_and_history() {
    use gridfabrica_ui_egui::i18n::Language;
    let mut h = harness(blank());
    h.state_mut().run("cell.set", json!({"cell": "A1", "input": "Home 中文"})).unwrap();
    let undo = h.state().session.active().unwrap().undo.len();
    h.state_mut().run("ui.language", json!({"language": "zh-CN"})).unwrap();
    h.run_steps(2);
    assert_eq!(h.state().ui.language, Language::Chinese);
    assert_eq!(value(&h, "A1"), Value::from("Home 中文"));
    assert_eq!(h.state().session.active().unwrap().undo.len(), undo);
    assert!(h.state_mut().run("ui.language", json!({"language": "unsupported"})).is_err());
    assert!(h.state_mut().run("ui.language", json!({})).is_err());
    assert_eq!(h.state().ui.language, Language::Chinese);
    let saved = serde_json::to_string(&h.state().ui).unwrap();
    let restored: gridfabrica_ui_egui::UiState = serde_json::from_str(&saved).unwrap();
    assert_eq!(restored.language, Language::Chinese);
    let legacy: gridfabrica_ui_egui::UiState = serde_json::from_str("{\"dark\":true}").unwrap();
    assert!(legacy.dark);
    assert_eq!(legacy.language, Language::English);
    h.state_mut().run("ui.language", json!({"language": "en"})).unwrap();
    h.run_steps(2);
    assert_eq!(h.state().ui.language, Language::English);
}

#[test]
fn language_picker_changes_rendered_labels_and_keeps_dialog_ids() {
    use egui_kittest::kittest::Queryable;
    use gridfabrica_ui_egui::i18n::Language;
    let mut h = harness(blank());
    h.get_by_value("English").click();
    h.run_steps(2);
    h.get_by_label("简体中文").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.language, Language::Chinese);
    h.state_mut().open_dialog("find", json!({}));
    h.run_steps(2);
    assert!(h.query_by_label("查找内容:").is_some());
    assert_eq!(h.state().dialog.as_ref().unwrap().name(), "find");
    h.state_mut().dialog = None;
    h.get_by_value("简体中文").click();
    h.run_steps(2);
    h.get_by_label("English").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.language, Language::English);
    h.state_mut().open_dialog("find", json!({}));
    h.run_steps(2);
    assert!(h.query_by_label("Find what:").is_some());
}

#[test]
fn chinese_ribbon_dialogs_and_text_input_render() {
    let mut h = harness(blank());
    h.state_mut().run("ui.language", json!({"language": "zh-CN"})).unwrap();
    for tab in gridfabrica_ui_egui::ribbon::TABS {
        h.state_mut().run("ui.ribbonTab", json!({"tab": tab})).unwrap();
        h.run_steps(2);
        assert_eq!(h.state().ui.ribbon_tab, *tab);
    }
    for name in ["formatCells", "dataValidation", "pasteSpecial", "sort", "find", "about"] {
        h.state_mut().open_dialog(name, json!({}));
        h.run_steps(2);
        assert_eq!(h.state().dialog.as_ref().unwrap().name(), name);
        h.state_mut().dialog = None;
    }
    // Exercise the egui text-input path. Native OS IME composition is tested separately.
    text(&mut h, "中文测试");
    key(&mut h, Key::Enter, Modifiers::NONE);
    assert_eq!(value(&h, "A1"), Value::from("中文测试"));
    if let Ok(dir) = std::env::var("GRIDFABRICA_I18N_SHOTS") {
        for (code, file) in [("zh-CN", "zh-CN.png"), ("en", "en.png")] {
            h.state_mut().run("ui.language", json!({"language": code})).unwrap();
            h.state_mut().ui.ribbon_tab = "Home".into();
            h.run_steps(3);
            h.render().unwrap().save(std::path::Path::new(&dir).join(file)).unwrap();
        }
    }
}

#[test]
fn chinese_faces_and_every_workbook_font_variant_have_glyphs() {
    let h = harness(blank());
    if !gridfabrica_ui_egui::theme::font_definitions().font_data.contains_key("cjk-fallback") {
        eprintln!("CJK font unavailable; run with CRAFT_FONTS_DIR to verify glyph coverage");
        return;
    }
    for name in [
        "Calibri",
        "宋体",
        "黑体",
        "微软雅黑",
        "楷体",
        "仿宋",
        "等线",
        "SimSun",
        "KaiTi",
        "FangSong",
        "Arial",
        "Times New Roman",
        "Consolas",
        "Noto Sans CJK SC",
        "Noto Serif CJK SC",
        "LXGW WenKai",
        "Zhuque Fangsong (technical preview)",
    ] {
        for (bold, italic) in [(false, false), (true, false), (false, true), (true, true)] {
            let font = egui::FontId::new(16.0, gridfabrica_ui_egui::theme::cell_family(name, bold, italic));
            // egui 0.36's has_glyph compares face keys, which reports false when a face also
            // supplies the replacement glyph. Compare actual rasterized glyphs instead.
            let replacement = h.ctx.fonts_mut(|fonts| fonts.layout_no_wrap("\u{10ffff}".into(), font.clone(), egui::Color32::WHITE));
            let missing_uv = replacement.rows.first().unwrap().glyphs.first().unwrap().uv_rect;
            for character in "名称中文数据北京上海".chars() {
                let galley = h.ctx.fonts_mut(|fonts| fonts.layout_no_wrap(character.to_string(), font.clone(), egui::Color32::WHITE));
                let glyph = galley.rows.first().unwrap().glyphs.first().unwrap();
                assert_ne!(glyph.uv_rect, missing_uv, "replacement glyph: {name}, bold={bold}, italic={italic}, char={character}");
            }
        }
    }
}

#[test]
fn common_chinese_font_choices_survive_xlsx_round_trip() {
    use egui_kittest::kittest::Queryable;
    let mut h = harness(blank());
    if !gridfabrica_ui_egui::theme::chinese_fonts(&h.ctx).contains(&"仿宋") {
        eprintln!("Chinese font pack unavailable; run packaging/chinese-fonts.py first");
        return;
    }
    h.state_mut().run("cell.set", json!({"cell":"A1", "input":"中文数据 ABC 123"})).unwrap();
    h.get_by_value("Calibri").click();
    h.run_steps(2);
    h.get_by_label("宋体").click();
    h.run_steps(2);
    assert_eq!(h.state_mut().run("cell.get", json!({"cell":"A1"})).unwrap()["style"]["font"]["name"], json!("宋体"));
    for name in ["宋体", "黑体", "楷体", "仿宋", "微软雅黑", "等线", "Noto Serif CJK SC", "LXGW WenKai", "Zhuque Fangsong (technical preview)"]
    {
        assert!(gridfabrica_ui_egui::theme::chinese_fonts(&h.ctx).contains(&name));
        h.state_mut().run("home.fontName", json!({"name":name})).unwrap();
        let saved = h.state_mut().run("file.saveBytes", json!({"format":"xlsx"})).unwrap();
        let mut opened = blank();
        opened.execute("file.open", json!({"name":"字体.xlsx", "base64":saved["base64"]})).unwrap();
        let cell = opened.execute("cell.get", json!({"cell":"A1"})).unwrap();
        assert_eq!(cell["style"]["font"]["name"], json!(name));
        assert_eq!(opened.active().unwrap().wb.active().unwrap().value(CellRef::parse("A1").unwrap()), Value::Text("中文数据 ABC 123".into()));
    }
    h.state_mut().open_dialog("formatCells", json!({"tab":"Font"}));
    h.run_steps(2);
    h.get_by_label("Format Cells").get_by_value("Zhuque Fangsong (technical preview)").click();
    h.run_steps(2);
    h.get_by_label("仿宋").click();
    h.run_steps(2);
    h.get_by_label("Format Cells").get_by_label_contains("OK").click();
    h.run_steps(2);
    assert_eq!(h.state_mut().run("cell.get", json!({"cell":"A1"})).unwrap()["style"]["font"]["name"], json!("仿宋"));
}

#[test]
fn choosing_chinese_font_sets_the_workbook_font_name() {
    use egui_kittest::kittest::Queryable;
    let mut h = harness(blank());
    if !gridfabrica_ui_egui::theme::chinese_fonts(&h.ctx).contains(&"Noto Sans CJK SC") {
        eprintln!("embedded Noto face unavailable; run with CRAFT_FONTS_DIR");
        return;
    }
    h.get_by_value("Calibri").click();
    h.run_steps(2);
    h.get_by_label("Noto Sans CJK SC").click();
    h.run_steps(2);
    assert_eq!(h.state_mut().run("cell.get", json!({"cell":"A1"})).unwrap()["style"]["font"]["name"], json!("Noto Sans CJK SC"));
}
