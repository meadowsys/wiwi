#![allow(
	dead_code,
	clippy::missing_safety_doc,
	reason = "wip"
)]
#![deny(
	unconditional_recursion,
	reason = "yes"
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
	pub use std::mem::{ ManuallyDrop, MaybeUninit };
	pub use std::ops::Deref;
	pub use wasm_bindgen::JsValue;

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
}
