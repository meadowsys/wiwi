use crate::prelude_internal::*;

/// Global `String` namespace
#[repr(transparent)]
pub struct ExternStringNs {
	inner: ExternObject
}

/// `string` primitive type
#[repr(transparent)]
pub struct ExternString {
	inner: ExternAny
}

/// `String` wrapper object
#[repr(transparent)]
pub struct ExternStringObject {
	inner: ExternObject
}
