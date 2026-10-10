//! Exercise the same file.open byte path used by browser uploads and desktop files.
use gridfabrica_engine::Session;
use serde_json::json;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn chinese_gbk_import_reaches_the_session_without_mojibake() {
    let bytes = b"\xc3\xfb\xb3\xc6,\xb1\xb8\xd7\xa2\n\xd6\xd0\xce\xc4,\xb2\xe2\xca\xd4\xca\xfd\xbe\xdd\n\xb1\xb1\xbe\xa9,100";
    let mut session = Session::new();
    session.execute("file.open", json!({"name": "中文.csv", "base64": gridfabrica_engine::io::base64_encode(bytes)})).unwrap();
    assert_eq!(session.execute("cell.get", json!({"cell": "A1"})).unwrap()["value"], json!("名称"));
    assert_eq!(session.execute("cell.get", json!({"cell": "A2"})).unwrap()["value"], json!("中文"));
    assert_eq!(session.execute("cell.get", json!({"cell": "B2"})).unwrap()["value"], json!("测试数据"));
    assert_eq!(session.execute("cell.get", json!({"cell": "B3"})).unwrap()["value"], json!(100.0));
}
