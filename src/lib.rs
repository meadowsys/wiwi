#![allow(
	dead_code,
	reason = "wip"
)]

pub use self::any::ExternAny;
// pub use self::bigint::ExternBigint;
// pub use self::boolean::ExternBoolean;
// pub use self::null::ExternNull;
// pub use self::number::ExternNumber;
pub use self::object::ExternObject;
// pub use self::reflect::ExternReflect;
// pub use self::string::ExternString;
// pub use self::symbol::ExternSymbol;
// pub use self::undefined::ExternUndefined;
pub use wasm_bindgen::prelude::wasm_bindgen;

// pub mod builder_api;
pub mod extern_crates;
mod sealed;

mod any;
// mod bigint;
// mod boolean;
// mod null;
// mod number;
mod object;
// mod reflect;
// mod string;
// mod symbol;
// mod undefined;

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
mod prelude_internal {
	pub(crate) use crate::{ ExternAny, wasm_bindgen };
	// pub(crate) use crate::builder_api::*;
	// pub(crate) use crate::sealed::Sealed;
	// pub(crate) use std::marker::PhantomData;
	// pub(crate) use std::mem::MaybeUninit;
	pub(crate) use std::ops::{ Deref, DerefMut };
	pub(crate) use wasm_bindgen::JsValue;

	// #[inline(always)]
	// pub(crate) fn uninit<T>() -> MaybeUninit<T> {
	// 	MaybeUninit::uninit()
	// }
}

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]
#[cfg(test)]
mod prelude_test {
	pub use std::hint::black_box;
	pub use wasm_bindgen_test::wasm_bindgen_test;
}
