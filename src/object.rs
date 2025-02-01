use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl ExternObject {
	#[inline]
	pub(crate) fn from_js_value_unchecked(value: JsValue) -> Self {
		let inner = ExternAny::from_js_value(value);
		Self { inner }
	}
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
