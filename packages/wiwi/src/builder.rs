//! Compile time checked builder APIs
//!
//! # (temporary) Checklist for manually writing builders
//!
//! - struct definition
//! - impl struct with `builder()` and `finish_init(..)`
//! - submodule for builder impl details
//!   - imports
//!   - pub type for init/uninit
//!   - builder struct def
//!   - builder state trait def
//!   - builder state container struct def
//!   - private mod for sealed trait
//!   - impl builder state trait
//!   - impl sealed
//!   - impl uninit for `new()` fn
//!   - impl<S> where S: builder state trait for all the fns including `build()`
//!     (`build()` calls `finish_init(..)`)
//!   - impl block, same as previous one in headers and stuffs, for the internal fns

extern crate wiwiwiwiwi;

use crate::prelude::*;
use self::private::*;

pub use wiwiwiwiwi::builder;

pub struct Uninit {
	__private: ()
}

pub trait IsUninit: IsUninitSealed {}

impl IsUninit for Uninit {}
impl IsUninitSealed for Uninit {}

pub struct Init {
	__private: ()
}

pub trait IsInit: IsInitSealed {}

impl IsInit for Init {}
impl IsInitSealed for Init {}

/// Compile-time known value for if the current type encodes initialised or not
///
/// This can be used for example for a state that has a default variant, where
/// it doesn't matter if it has been set or not before buliding. However, we
/// cannot call `assume_init` on uninitialised values! We can therefore use this
/// value to check if something has been initialised (and the type changed to
/// reflect it), and act accordingly.
///
/// Since the value is a constant known at compile time, the optimiser is
/// very likely able to elide the check.
///
/// # Safety
///
/// Implementing this trait is a promise that your
/// [`IS_INIT`](InitialisationStatus::IS_INIT)
/// value is actually reflective of initialisation state, and
/// [`InvertInitialisationStatus`](InitialisationStatus::InvertInitialisationStatus)
/// is too, assuming the types have been used correctly.
pub unsafe trait InitialisationStatus: InitialisationStatusSealed {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
	type InvertInitialisationStatus: InitialisationStatus;
}

// SAFETY: `Uninit` represents being uninitialised
unsafe impl InitialisationStatus for Uninit {
	const IS_INIT: bool = false;
	type InvertInitialisationStatus = Init;
}

impl InitialisationStatusSealed for Uninit {}

// SAFETY: `Init` represents being initialised
unsafe impl InitialisationStatus for Init {
	const IS_INIT: bool = true;
	type InvertInitialisationStatus = Uninit;
}

impl InitialisationStatusSealed for Init {}

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;

/// notouchie
mod private {
	/// notouchie
	pub trait IsUninitSealed {}
	/// notouchie
	pub trait IsInitSealed {}
	/// notouchie
	pub trait InitialisationStatusSealed {}
}
