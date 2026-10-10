//! Load versioned Chinese font resources independently of the WASM binary.
use wasm_bindgen::{JsCast as _, closure::Closure};
use wasm_bindgen_futures::JsFuture;

/// Fetch all configured faces concurrently. Individual network failures leave the remaining
/// faces usable and are returned for the shell to report. Requests expire after 30 seconds.
pub async fn load() -> (Vec<(&'static str, Vec<u8>)>, Vec<String>) {
    let requests = gridfabrica_ui_egui::theme::WEB_FONT_ASSETS.iter().map(|&(id, url, size)| async move {
        fetch(url, size).await.map(|bytes| (id, bytes)).map_err(|error| {
            log::warn!("{error}");
            match id {
                "CJK_FONT" => "Noto Sans CJK SC",
                "CJK_SERIF_FONT" => "Noto Serif CJK SC",
                "CJK_KAI_FONT" => "LXGW WenKai",
                "CJK_FANG_FONT" => "Zhuque Fangsong (technical preview)",
                _ => id,
            }
            .to_string()
        })
    });
    let mut fonts = Vec::new();
    let mut errors = Vec::new();
    for result in futures_util::future::join_all(requests).await {
        match result {
            Ok(font) => fonts.push(font),
            Err(error) => errors.push(error),
        }
    }
    (fonts, errors)
}

async fn fetch(url: &str, expected: u64) -> Result<Vec<u8>, String> {
    if expected == 0 || expected > 64 * 1024 * 1024 {
        return Err(format!("Invalid font size: {url}"));
    }
    let window = web_sys::window().ok_or("No browser window for font loading")?;
    let controller = web_sys::AbortController::new().map_err(|_| "Font request cancellation unavailable")?;
    let abort = controller.clone();
    let timeout = Closure::<dyn FnMut()>::new(move || abort.abort());
    let timer = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.as_ref().unchecked_ref(), 30_000)
        .map_err(|_| "Font request timeout unavailable")?;
    let init = web_sys::RequestInit::new();
    init.set_signal(Some(&controller.signal()));
    let result = async {
        let value =
            JsFuture::from(window.fetch_with_str_and_init(url, &init)).await.map_err(|_| format!("Font request failed or timed out: {url}"))?;
        let response = value.dyn_into::<web_sys::Response>().map_err(|_| format!("Invalid font response: {url}"))?;
        if !response.ok() {
            return Err(format!("Font request returned HTTP {}: {url}", response.status()));
        }
        let buffer = response.array_buffer().map_err(|_| format!("Could not read font response: {url}"))?;
        let buffer = JsFuture::from(buffer).await.map_err(|_| format!("Font download interrupted: {url}"))?;
        let bytes = js_sys::Uint8Array::new(&buffer);
        if u64::from(bytes.length()) != expected {
            return Err(format!("Font resource has an unexpected size: {url}"));
        }
        Ok(bytes.to_vec())
    }
    .await;
    window.clear_timeout_with_handle(timer);
    result
}
