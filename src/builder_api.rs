use crate::internal_prelude::*;

// checklist™
// - struct definition
// - impl struct with `builder()` and `finish_init(..)`
// - submodule for builder impl details
//   - imports
//   - pub type for init/uninit
//   - builder struct def
//   - builder state trait def
//   - builder state container struct def
//   - impl builder state trait
//   - impl uninit for `new()` fn
//   - impl<S> where S: builder state trait for all the fns including `build()`
//     (`build()` calls `finish_init(..)`)
//   - impl block, same as previous one in headers and stuffs, for the internal fns

pub struct Init {
	__private: ()
}

pub struct Uninit {
	__private: ()
}

// todo ???
#[diagnostic::on_unimplemented(
	message = "a required field is not initialised",
	label = "this field is not initialised",
	note = "call a builder method for this field before building"
)]
pub trait IsInit {}

impl IsInit for Init {}

// todo ???
#[diagnostic::on_unimplemented(
	message = "a field is already initialised",
	label = "this field is already initialised",
	note = "omit this builder method call, or a previous builder method call that has set this field, or call a builder method to erase this field"
)]
pub trait IsUninit {}

impl IsUninit for Uninit {}

pub trait InitStatus {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
}

impl InitStatus for Init {
	const IS_INIT: bool = true;
}

impl InitStatus for Uninit {
	const IS_INIT: bool = false;
}

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;
