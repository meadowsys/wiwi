// todo this file is incomplete

use crate::prelude_internal::*;

#[doc(inline)]
pub use ns::ExternObjectNs;

pub mod ns;
mod raw;

gen_struct! {
	struct ExternObject;

	deref ExternAny;
	deref_value todo!();
}

gen_state! {}
