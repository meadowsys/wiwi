#[repr(transparent)]
pub struct ExternValue {
	inner: wasm_bindgen::JsValue
}

impl ExternValue {
	#[inline]
	pub fn as_sys(&self) -> &wasm_bindgen::JsValue {
		&self.inner
	}

	#[inline]
	pub fn into_sys(self) -> wasm_bindgen::JsValue {
		self.inner
	}
}
