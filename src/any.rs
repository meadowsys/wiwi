use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternAny {
	inner: JsValue
}

impl ExternAny {
	#[inline(always)]
	pub fn from_js_value(value: JsValue) -> Self {
		Self { inner: value }
	}

	#[inline(always)]
	pub fn from_js_value_ref(value: &JsValue) -> &Self {
		// SAFETY: ExternAny is repr(transparent) over JsValue
		unsafe { &*(&raw const *value as *const ExternAny) }
	}

	#[inline(always)]
	pub fn as_js_value(&self) -> &JsValue {
		&self.inner
	}

	#[inline(always)]
	pub fn into_js_value(self) -> JsValue {
		self.inner
	}
}
