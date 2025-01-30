#![allow(
	dead_code,
	reason = "wip"
)]

pub use wasm_bindgen::prelude::wasm_bindgen;

pub mod builder_api;
pub mod extern_crates;

macro_rules! do_export {
	{ $($vis:vis $mod:ident::$ident:ident)* } => {
		$(
			$vis mod $mod;
			pub use self::$mod::$ident;
		)*
	}
}

do_export! {
	any::ExternAny
	bigint::ExternBigint
	boolean::ExternBoolean
	number::ExternNumber
	string::ExternString
	symbol::ExternSymbol
}
