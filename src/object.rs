use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl ExternObject {
	#[inline]
	pub unsafe fn from_any_unchecked(value: ExternAny) -> Self {
		Self { inner: value }
	}

	#[inline]
	pub unsafe fn from_any_ref_unchecked(value: &ExternAny) -> &Self {
		// SAFETY: ExternObject is repr(transparent) over ExternAny
		unsafe { &*(&raw const value as *const ExternObject) }
	}

	#[inline]
	pub unsafe fn from_js_value_unchecked(value: JsValue) -> Self {
		let value = ExternAny::from_js_value(value);
		// SAFETY: caller promises provided `value` is actually an object
		unsafe { Self::from_any_unchecked(value) }
	}

	#[inline]
	pub unsafe fn from_js_value_ref_unchecked(value: &JsValue) -> &Self {
		let value = ExternAny::from_js_value_ref(value);
		// SAFETY: caller promises provided `value` is actually an object
		unsafe { Self::from_any_ref_unchecked(value) }
	}

	#[inline]
	pub fn as_any(&self) -> &ExternAny {
		&self.inner
	}
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline]
	fn deref(&self) -> &ExternAny {
		self.as_any()
	}
}
