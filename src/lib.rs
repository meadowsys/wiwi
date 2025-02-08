#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
	dead_code,
	unused_imports,
	clippy::missing_safety_doc,
	reason = "wip (todo remove me)"
)]
#![deny(
	unconditional_recursion,
	clippy::missing_inline_in_public_items,
	reason = "yes"
)]

#![cfg_attr(all(docsrs, kiwingay), doc = "")]
#![cfg_attr(
	all(docsrs, kiwingay),
	doc = concat!(
		"These docs have been built from commit [",
		env!("KIWINGAY_DEPLOY_COMMIT_SHORT"),
		"](https://github.com/meadowsys/wiwi-wasm/commit/",
		env!("KIWINGAY_DEPLOY_COMMIT"),
		")."
	)
)]

pub use self::any::ExternAny;
pub use self::bigint::{ ExternBigint, ExternBigintNs, bigint };
pub use self::object::ExternObject;
pub use self::number::{ ExternNumber, ExternNumberNs, ExternNumberObject, number };
#[doc(inline)]
pub use self::reflect::{ ExternReflectNs, reflect };
pub use self::string::{ ExternString, ExternStringNs, ExternStringObject };
pub use wasm_bindgen::prelude::wasm_bindgen;

pub mod extern_crates;
pub mod slot;
pub mod util;

mod any;
mod bigint;
mod boolean;
mod object;
mod number;
pub mod reflect;
mod string;
mod symbol;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
mod prelude_internal {
	pub(crate) use crate::{ ExternAny, ExternObject, wasm_bindgen };
	pub(crate) use crate::util::*;
	pub(crate) use crate::slot::*;
	pub(crate) use core::mem::{ ManuallyDrop, MaybeUninit, transmute };
	pub(crate) use core::ops::Deref;
}

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
#[cfg(test)]
mod prelude_test {
	pub(crate) use wasm_bindgen_test::wasm_bindgen_test;
	pub(crate) use core::hint::black_box;
}
