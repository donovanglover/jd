use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn testing() -> JsValue {
    JsValue::from_str("test")
}
