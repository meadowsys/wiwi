use crate::prelude_internal::*;

#[repr(transparent)]
pub struct ExternStringNs {
	inner: ExternObject
}

#[repr(transparent)]
pub struct ExternString {
	inner: ExternAny
}

#[repr(transparent)]
pub struct ExternStringObject {
	inner: ExternObject
}
