#![allow(
	dead_code,
	clippy::missing_safety_doc,
	reason = "wip"
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

pub mod builder;
mod sealed;

mod any;
mod bigint;
mod boolean;
mod object;
mod number;
pub mod reflect;
mod string;

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
mod prelude_internal {
	pub use crate::{ ExternAny, ExternObject, wasm_bindgen };
	pub use crate::builder::*;
	pub use crate::sealed::Sealed;
	pub use std::mem::{ ManuallyDrop, MaybeUninit, transmute };
	pub use std::ops::Deref;
	pub use wasm_bindgen::JsValue;

	#[deprecated(note = "nei")]
	#[inline]
	pub fn uninit<T>() -> MaybeUninit<T> {
		MaybeUninit::uninit()
	}
}

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
#[cfg(test)]
mod prelude_test {
	pub use wasm_bindgen_test::wasm_bindgen_test;
	pub use std::hint::black_box;
}
