use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternNull {
	inner: ExternAny
}

impl Deref for ExternNull {
	type Target = ExternAny;

	#[inline(always)]
	fn deref(&self) -> &ExternAny {
		&self.inner
	}
}
