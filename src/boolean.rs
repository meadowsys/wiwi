use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternBoolean {
	inner: ExternAny
}

impl Deref for ExternBoolean {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
