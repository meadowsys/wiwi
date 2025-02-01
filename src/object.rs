use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternObject {
	inner: ExternAny
}

impl Deref for ExternObject {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
