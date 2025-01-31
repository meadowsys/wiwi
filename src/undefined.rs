use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternUndefined {
	inner: ExternAny
}

impl Deref for ExternUndefined {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
