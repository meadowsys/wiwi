use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternNumber {
	inner: ExternAny
}

impl ExternNumber {
	#[inline(always)]
	pub fn is_finite(&self) -> bool {
		// js_sys::Number::is_finite(&self.inner)
		todo!()
	}

	#[inline(always)]
	pub fn is_integer(&self) -> bool {
		// js_sys::Number::is_integer(&self.inner)
		todo!()
	}

	#[inline(always)]
	pub fn is_nan(&self) -> bool {
		// js_sys::Number::is_nan(&self.inner)
		todo!()
	}

	#[inline(always)]
	pub fn is_safe_integer(&self) -> bool {
		// js_sys::Number::is_safe_integer(&self.inner)
		todo!()
	}
}

impl Deref for ExternNumber {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
