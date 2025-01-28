/// Common builder API stuffs
///
/// # Checklist™
///
/// - struct definition (outside the impl mod) mimicing the function call name
/// - use statements
/// - impl block on the struct for "starting" builder methods
/// - builder struct (with repr(transparent), `inner`, `__marker`)
/// - builder inner struct
/// - state trait
/// - state container struct
/// - type definitions for important states
/// - impl state for statecontainer
/// - impl builder uninit
/// - impl blocks for finished ones with `call` or `build` fns
/// - impl block for builder fns, `change_state`, internal functions etc

use crate::internal_prelude::*;

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
