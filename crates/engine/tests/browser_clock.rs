//! Workbook commits must survive the wasm clock path, including sample startup.
use gridfabrica_engine::Session;
use serde_json::json;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn workbook_edits_use_a_supported_clock() {
    let mut session = Session::new();
    session.execute("file.new", json!({"sample": "sales"})).unwrap();
    session.execute("cell.set", json!({"cell": "Z1", "input": "=SUM(1,2,3)"})).unwrap();
    let result = session.execute("cell.get", json!({"cell": "Z1"})).unwrap();
    assert_eq!(result["value"], json!(6.0));
}
