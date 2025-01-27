#[repr(transparent)]
pub struct ExternStr {
	inner: wasm_bindgen::JsValue
}
