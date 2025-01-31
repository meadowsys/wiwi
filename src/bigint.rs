use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternBigint {
	inner: ExternAny
}

impl Deref for ExternBigint {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
