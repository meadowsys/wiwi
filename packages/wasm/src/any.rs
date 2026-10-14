#[repr(transparent)]
pub struct ExternAny {
	inner: wasm_bindgen::JsValue
}

impl ExternAny {
	const NULL: Self = Self::from_js_value(wasm_bindgen::JsValue::NULL);
	const UNDEFINED: Self = Self::from_js_value(wasm_bindgen::JsValue::UNDEFINED);
	const TRUE: Self = Self::from_js_value(wasm_bindgen::JsValue::TRUE);
	const FALSE: Self = Self::from_js_value(wasm_bindgen::JsValue::FALSE);

	#[inline]
	pub const fn from_js_value(value: wasm_bindgen::JsValue) -> Self {
		Self { inner: value }
	}

	#[inline]
	pub const fn from_js_value_ref(value: &wasm_bindgen::JsValue) -> &Self {
		// SAFETY: ExternAny is repr(transparent) over JsValue
		unsafe { &*(&raw const *value).cast() }
	}

	#[inline]
	pub const fn as_js_value(&self) -> &wasm_bindgen::JsValue {
		&self.inner
	}

	#[inline]
	pub fn into_js_value(self) -> wasm_bindgen::JsValue {
		self.inner
	}

	#[inline]
	pub const fn new_null() -> Self {
		Self::NULL
	}

	#[inline]
	pub const fn new_undefined() -> Self {
		Self::UNDEFINED
	}

	#[inline]
	pub const fn new_true() -> Self {
		Self::TRUE
	}

	#[inline]
	pub const fn new_false() -> Self {
		Self::FALSE
	}

	#[inline]
	pub fn new_symbol(desc: Option<&str>) -> Self {
		Self::from_js_value(wasm_bindgen::JsValue::symbol(desc))
	}

	#[expect(clippy::should_implement_trait, reason = "yeag")]
	#[inline]
	pub fn from_str(value: &str) -> Self {
		Self::from_js_value(wasm_bindgen::JsValue::from_str(value))
	}

	#[inline]
	pub fn from_f64(value: f64) -> Self {
		Self::from_js_value(wasm_bindgen::JsValue::from_f64(value))
	}
}
