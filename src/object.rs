// todo this file is incomplete

use crate::prelude_internal::*;

#[doc(inline)]
pub use ns::{ ExternObjectNs, ExternObjectNsWithState };

pub mod ns;
mod raw;

gen_struct! {
	struct ExternObject ExternObjectWithState;

	deref ExternAny;
	deref_value todo!();
}

gen_state! {}
