#![allow(
	dead_code,
	unused_imports,
	reason = "prelude"
)]

pub use std::marker::PhantomData;
pub use std::mem::{ ManuallyDrop, MaybeUninit };

pub use wasm_bindgen::JsValue;

pub fn uninit<T>() -> MaybeUninit<T> {
	MaybeUninit::uninit()
}
