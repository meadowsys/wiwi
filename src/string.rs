use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternString {
	inner: ExternAny
}

impl Deref for ExternString {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
