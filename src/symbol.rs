use crate::prelude_internal::*;

/// Global `Symbol` namespace
#[repr(transparent)]
pub struct ExternSymbolNs {
	inner: ExternObject
}

/// `symbol` primitive type
#[repr(transparent)]
pub struct ExternSymbol {
	inner: ExternAny
}
