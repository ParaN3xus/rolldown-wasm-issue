use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WasmFuncCallParameters {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub f: js_sys::Function,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub v: js_sys::Number,
}

#[wasm_bindgen]
pub fn f(p: JsValue) -> Result<f64, JsValue> {
    let p = serde_wasm_bindgen::from_value::<WasmFuncCallParameters>(p)
        .map_err(|err| JsValue::from_str(&format!("Failed to deserialize init opts: {err}")))?;

    return p.v.as_f64().ok_or(JsValue::from_str("Not a f64"));
}
