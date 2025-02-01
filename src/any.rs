use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternAny {
	inner: JsValue
}

impl ExternAny {
	#[inline(always)]
	pub(crate) fn from_js_value(value: JsValue) -> Self {
		Self { inner: value }
	}

	#[inline(always)]
	pub fn as_js_value(&self) -> &JsValue {
		&self.inner
	}

	#[inline(always)]
	pub fn as_mut_js_value(&mut self) -> &mut JsValue {
		&mut self.inner
	}
}
