#[repr(transparent)]
pub struct ExternAny {
	inner: wasm_bindgen::JsValue
}

impl ExternAny {
	#[inline(always)]
	pub fn as_js_value(&self) -> &wasm_bindgen::JsValue {
		&self.inner
	}
}
