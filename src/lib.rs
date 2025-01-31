#![allow(
	dead_code,
	reason = "wip"
)]

pub use self::any::ExternAny;
pub use self::bigint::ExternBigint;
pub use self::boolean::ExternBoolean;
pub use self::null::ExternNull;
pub use self::number::ExternNumber;
pub use self::object::ExternObject;
pub use self::reflect::ExternReflect;
pub use self::string::ExternString;
pub use self::symbol::ExternSymbol;
pub use self::undefined::ExternUndefined;
pub use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub mod builder_api;
pub mod extern_crates;
mod prelude_internal;
mod sealed;

mod any;
mod bigint;
mod boolean;
mod null;
mod number;
mod object;
mod reflect;
mod string;
mod symbol;
mod undefined;
