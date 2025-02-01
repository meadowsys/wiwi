pub use std::marker::PhantomData;

pub struct Uninit {
	__private: ()
}

pub struct Init {
	__private: ()
}

pub trait IsUninit {}
impl IsUninit for Uninit {}

pub trait IsInit {}
impl IsInit for Init {}

/// Trait for marker structs to hold state about if a field in
/// a builder is initialised or not
///
/// # Safety
///
/// [`IS_INIT`](InitStatus::IS_INIT) and [`IS_UNINIT`](InitStatus::IS_UNINIT)
/// must both be set correctly to acccurately represent the state of the field.
pub unsafe trait InitStatus {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
}

unsafe impl InitStatus for Uninit {
	const IS_INIT: bool = false;
}

unsafe impl InitStatus for Init {
	const IS_INIT: bool = true;
}

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;
