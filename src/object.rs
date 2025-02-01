use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl ExternObject {
	#[inline(always)]
	pub(crate) fn from_extern_any_unchecked(value: ExternAny) -> Self {
		Self { inner: value }
	}

	#[inline(always)]
	pub(crate) fn from_extern_any_ref_unchecked(value: &ExternAny) -> &Self {
		// SAFETY: ExternObject is repr(transparent) over ExternAny
		unsafe { &*(&raw const value as *const ExternObject) }
	}

	#[inline(always)]
	pub(crate) fn from_js_value_unchecked(value: JsValue) -> Self {
		let value = ExternAny::from_js_value(value);
		Self::from_extern_any_unchecked(value)
	}

	#[inline(always)]
	pub(crate) fn from_js_value_ref_unchecked(value: &JsValue) -> &Self {
		let value = ExternAny::from_js_value_ref(value);
		Self::from_extern_any_ref_unchecked(value)
	}
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
