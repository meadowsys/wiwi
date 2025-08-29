//! moo
//!
//! Deserialise implementations are done individually rather than a blanket impl.
//! This is because some things (like strings) can be and will benefit from zero
//! copy deserialisation (ie. deserialising to Cow::Borrowed), but other things
//! (ex. arrays of assorted compressed integers) cannot be. This hypothetical
//! blanket deser impl could be done by always serialising to the owned variant
//! (ie. deserialising to Cow::Owned), but types that can zero copy deserialise
//! lose out on the ability to.
//!
//! Rust! Specialisation when?

use super::internal_prelude::*;
use std::borrow::Cow;

impl<T: ?Sized + Serialise + ToOwned> Serialise for Cow<'_, T> {
	type Serialiser<'h> = T::Serialiser<'h> where Self: 'h;

	fn build_serialiser(&self) -> T::Serialiser<'_> {
		(**self).build_serialiser()
	}
}
