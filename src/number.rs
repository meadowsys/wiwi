use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternNumberNs {
	inner: ExternObject
}

#[repr(transparent)]
pub struct ExternNumber {
	inner: ExternAny
}

#[repr(transparent)]
pub struct ExternNumberObject {
	inner: ExternObject
}
