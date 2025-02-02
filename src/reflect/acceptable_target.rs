use crate::prelude_internal::*;

pub trait AcceptableTarget {
	fn as_target(&self) -> &ExternObject;
}
