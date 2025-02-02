use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternBigintNs {
	inner: ExternObject
}

#[repr(transparent)]
pub struct ExternBigint {
	inner: ExternAny
}

// todo ??
// pub struct ExternBigintObject {}
