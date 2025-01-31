use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternString {
	inner: ExternAny
}
