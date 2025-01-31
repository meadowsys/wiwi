use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternSymbol {
	inner: ExternAny
}

impl Deref for ExternSymbol {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
