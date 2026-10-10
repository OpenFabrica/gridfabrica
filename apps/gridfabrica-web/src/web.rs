//! The browser shell: web `Services`, drag-and-drop, and the eframe web runner.

use gridfabrica_engine::Session;
use gridfabrica_ui_egui::{Inbox, Services, SheetApp};
use wasm_bindgen::JsCast as _;

#[path = "fonts.rs"]
mod fonts;

const CANVAS_ID: &str = "gridfabrica_canvas";
const LOADING_ID: &str = "gridfabrica_loading";

pub fn start() {
    eframe::WebLogger::init(log::LevelFilter::Info).ok();
    wasm_bindgen_futures::spawn_local(async {
        let Some(document) = web_sys::window().and_then(|w| w.document()) else {
            log::error!("no document");
            return;
        };
        let Some(canvas) = document.get_element_by_id(CANVAS_ID).and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok()) else {
            log::error!("missing <canvas id=\"{CANVAS_ID}\">");
            return;
        };
        let mut options = eframe::WebOptions::default();
        let (fonts, font_errors) = fonts::load().await;
        if fonts.is_empty() {
            log::error!("No font resources available: {}", font_errors.join("; "));
            if let Some(el) = document.get_element_by_id(LOADING_ID) {
                el.set_text_content(Some("字体资源未能加载，请检查网络连接或站点的 fonts/ 目录，然后刷新重试。 Font resources unavailable; check the connection and refresh."));
            }
            return;
        }
        if query().contains("webgl")
            && let eframe::egui_wgpu::WgpuSetup::CreateNew(create) = &mut options.wgpu_options.wgpu_setup
        {
            create.instance_descriptor.backends = eframe::wgpu::Backends::GL;
        }
        let result = eframe::WebRunner::new()
            .start(
                canvas,
                options,
                Box::new(move |cc| {
                    let inbox: Inbox = Inbox::default();
                    let mut session = Session::new();
                    let q = query();
                    if let Some(name) = q.split(['?', '&']).find_map(|kv| kv.strip_prefix("sample=")) {
                        let _ = session.execute("file.new", serde_json::json!({"sample": name}));
                    } else if q.contains("sample") {
                        let _ = session.execute("file.new", serde_json::json!({"sample": "sales"}));
                    }
                    let mut app = SheetApp::new(session, services(inbox.clone(), cc.egui_ctx.clone()));
                    let saved = load_language();
                    if let Some(language) = saved {
                        app.ui.language = language;
                    }
                    app.ui.language.install(&cc.egui_ctx);
                    if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
                        let _ = root.set_attribute("lang", app.ui.language.code());
                    }
                    gridfabrica_ui_egui::theme::set_runtime_fonts(&cc.egui_ctx, fonts)?;
                    SheetApp::setup_context(&cc.egui_ctx, false);
                    if !font_errors.is_empty() {
                        app.message = Some((
                            app.ui.language.text("Font resources unavailable"),
                            app.ui.language.format(
                                "Some Chinese fonts could not be loaded. Check the connection and refresh to retry.\n{details}",
                                &[("details", &font_errors.iter().map(|name| app.ui.language.text(name)).collect::<Vec<_>>().join("、"))],
                            ),
                        ));
                    }
                    Ok(Box::new(WebShell { app, inbox, last_language: saved }))
                }),
            )
            .await;
        if let Some(el) = document.get_element_by_id(LOADING_ID) {
            match result {
                Ok(()) => el.remove(),
                Err(e) => el.set_inner_html(&format!("<p>GridFabrica failed to start: {e:?}</p><p>A browser with WebGPU or WebGL2 is required.</p>")),
            }
        }
    });
}

fn query() -> String {
    web_sys::window().and_then(|w| w.location().search().ok()).unwrap_or_default()
}

struct WebShell {
    app: SheetApp,
    inbox: Inbox,
    last_language: Option<gridfabrica_ui_egui::i18n::Language>,
}

impl eframe::App for WebShell {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dropped = ctx.input_mut(|i| std::mem::take(&mut i.raw.dropped_files));
        for f in dropped {
            let inbox = self.inbox.clone();
            let ctx = ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let name = f.path().file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "dropped.xlsx".into());
                match f.bytes_async().await {
                    Ok(bytes) => {
                        inbox.lock().unwrap_or_else(|e| e.into_inner()).push((name, bytes));
                        ctx.request_repaint();
                    }
                    Err(e) => log::error!("couldn't read dropped file {name}: {e}"),
                }
            });
        }
        self.app.logic(ctx);
        if self.last_language != Some(self.app.ui.language) {
            if let Err(error) = save_language(self.app.ui.language) {
                log::warn!("{error}");
            }
            // Storage may be disabled. Keep the session language, but attempt each change
            // only once instead of writing/logging on every animation frame.
            self.last_language = Some(self.app.ui.language);
        }
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw: &mut egui::RawInput) {
        self.app.raw_input_hook(raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app.ui(ui);
    }
}

fn services(inbox: Inbox, ctx: egui::Context) -> Services {
    let open_inbox = inbox.clone();
    Services {
        open_async: Some(Box::new(move || {
            let inbox = open_inbox.clone();
            let ctx = ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let Some(file) =
                    rfd::AsyncFileDialog::new().add_filter("Spreadsheets", &["xlsx", "xlsm", "csv", "tsv", "txt", "json"]).pick_file().await
                else {
                    return;
                };
                let bytes = file.read().await;
                inbox.lock().unwrap_or_else(|e| e.into_inner()).push((file.file_name(), bytes));
                ctx.request_repaint();
            });
        })),
        download: Some(Box::new(|name: &str, bytes: &[u8]| {
            if let Err(e) = download(name, bytes) {
                log::error!("download of {name} failed: {e}");
            }
        })),
        open_url: Some(Box::new(|url: &str| {
            if let Some(w) = web_sys::window() {
                let _ = w.open_with_url_and_target(url, "_blank");
            }
        })),
        inbox: Some(inbox),
        ..Default::default()
    }
}

/// Triggers a browser download of `bytes`.
fn download(name: &str, bytes: &[u8]) -> Result<(), String> {
    let js = |e: wasm_bindgen::JsValue| format!("{e:?}");
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let parts = js_sys::Array::of1(&js_sys::Uint8Array::from(bytes));
    let opts = web_sys::BlobPropertyBag::new();
    opts.set_type(if name.ends_with(".csv") { "text/csv" } else { "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" });
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(&parts, &opts).map_err(js)?;
    let url = web_sys::Url::create_object_url_with_blob(&blob).map_err(js)?;
    let a: web_sys::HtmlAnchorElement = document.create_element("a").map_err(js)?.dyn_into().map_err(|_| "not an anchor")?;
    a.set_href(&url);
    a.set_download(name);
    a.style().set_property("display", "none").map_err(js)?;
    let body = document.body().ok_or("no body")?;
    body.append_child(&a).map_err(js)?;
    a.click();
    a.remove();
    let revoke = wasm_bindgen::closure::Closure::once_into_js(move || {
        web_sys::Url::revoke_object_url(&url).ok();
    });
    window.set_timeout_with_callback_and_timeout_and_arguments_0(revoke.unchecked_ref(), 10_000).map_err(js)?;
    Ok(())
}

const LANGUAGE_KEY: &str = "gridfabrica.interfaceLanguage";

fn load_language() -> Option<gridfabrica_ui_egui::i18n::Language> {
    let storage = web_sys::window()?.local_storage().ok()??;
    [LANGUAGE_KEY, "gridcraft.interfaceLanguage"].into_iter().find_map(|key| {
        let code = storage.get_item(key).ok()??;
        let language = gridfabrica_ui_egui::i18n::Language::parse(&code).ok()?;
        if key != LANGUAGE_KEY && storage.set_item(LANGUAGE_KEY, language.code()).is_err() {
            log::warn!("Could not migrate interface language preference");
        }
        Some(language)
    })
}

fn save_language(language: gridfabrica_ui_egui::i18n::Language) -> Result<(), String> {
    let window = web_sys::window().ok_or("language preference: no browser window")?;
    if let Some(root) = window.document().and_then(|document| document.document_element()) {
        let _ = root.set_attribute("lang", language.code());
    }
    let storage = window.local_storage().map_err(|_| "language preference storage unavailable")?.ok_or("language preference storage disabled")?;
    storage.set_item(LANGUAGE_KEY, language.code()).map_err(|_| "could not save language preference".into())
}
