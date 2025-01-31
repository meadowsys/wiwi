#[repr(transparent)]
pub struct ExternAny {
	inner: wasm_bindgen::JsValue
}
