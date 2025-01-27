#[repr(transparent)]
pub struct ExternObj {
	inner: wasm_bindgen::JsValue
}
