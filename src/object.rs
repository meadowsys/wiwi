use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl ExternObject {
	#[inline(always)]
	pub(crate) fn from_js_value_unchecked(value: JsValue) -> Self {
		let inner = ExternAny::from_js_value(value);
		Self { inner }
	}

	#[inline(always)]
	pub(crate) fn from_js_value_ref_unchecked(value: &JsValue) -> &Self {
		let value = ExternAny::from_js_value_ref(value);
		// SAFETY: ExternObject is repr(transparent) over ExternAny
		unsafe { &*(&raw const value as *const ExternObject) }
	}
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
