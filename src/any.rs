use wasm_bindgen::JsValue;

#[repr(transparent)]
pub struct ExternAny {
	inner: JsValue
}

impl ExternAny {
	#[inline]
	pub fn from_js_value(value: JsValue) -> Self {
		Self { inner: value }
	}

	#[inline]
	pub fn from_js_value_ref(value: &JsValue) -> &Self {
		// SAFETY: ExternAny is repr(transparent) over JsValue
		unsafe { &*(&raw const *value as *const ExternAny) }
	}

	#[inline]
	pub fn as_js_value(&self) -> &JsValue {
		&self.inner
	}

	#[inline]
	pub fn into_js_value(self) -> JsValue {
		self.inner
	}

	#[inline]
	#[expect(
		clippy::should_implement_trait,
		reason = "shshshhshhshhshsh"
	)]
	pub fn from_str(s: &str) -> Self {
		Self::from_js_value(JsValue::from_str(s))
	}
}
