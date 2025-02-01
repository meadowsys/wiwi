#![allow(
	dead_code,
	reason = "wip"
)]
#![deny(
	unconditional_recursion,
	reason = "yes"
)]

pub use self::any::ExternAny;
pub use wasm_bindgen::prelude::wasm_bindgen;

mod any;
mod object;

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
mod prelude_internal {
	pub use crate::{ ExternAny, wasm_bindgen };
	pub use std::mem::MaybeUninit;
	pub use std::ops::Deref;
	pub use wasm_bindgen::JsValue;

	#[inline(always)]
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
