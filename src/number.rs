pub struct ExternNumber {
	inner: wasm_bindgen::JsValue
}

impl ExternNumber {
	#[inline(always)]
	pub fn is_finite(&self) -> bool {
		js_sys::Number::is_finite(&self.inner)
	}

	#[inline(always)]
	pub fn is_integer(&self) -> bool {
		js_sys::Number::is_integer(&self.inner)
	}

	#[inline(always)]
	pub fn is_nan(&self) -> bool {
		js_sys::Number::is_nan(&self.inner)
	}

	#[inline(always)]
	pub fn is_safe_integer(&self) -> bool {
		js_sys::Number::is_safe_integer(&self.inner)
	}
}
