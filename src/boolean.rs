use crate::prelude_internal::*;

/// Global `Boolean` namespace
#[repr(transparent)]
pub struct ExternBooleanNs {
	inner: ExternAny
}

/// `boolean` primitive type
#[repr(transparent)]
pub struct ExternBoolean {
	inner: ExternAny
}

/// `Boolean` wrapper object
#[repr(transparent)]
pub struct ExternBooleanObject {
	inner: ExternAny
}
