//! Compile time checked builder APIs

// # (temporary) Checklist for manually writing builders
//
// - struct definition
// - impl struct with `builder()` and `finish_init(..)`
// - submodule for builder impl details
//   - imports
//   - pub type for init/uninit
//   - builder struct def
//   - builder state trait def
//   - builder state container struct def
//   - private mod for sealed trait
//   - impl builder state trait
//   - impl sealed
//   - impl uninit for `new()` fn
//   - impl<S> where S: builder state trait for all the fns including `build()`
//     (`build()` calls `finish_init(..)`)
//   - impl block, same as previous one in headers and stuffs, for the internal fns

use crate::prelude::*;

pub struct Uninit {
	__private: ()
}

pub struct Init<T = ()>
where
	T: ?Sized
{
	__marker: PhantomDataInvariant<T>
}

/// Trait for marker structs to hold state about if a field in
/// a builder is initialised or not
///
/// # Safety
///
/// [`IS_INIT`] and [`IS_UNINIT`] must both be set correctly to acccurately
/// represent the state of the field. If you set [`IS_INIT`] correctly, there is
/// a default implementation for [`IS_UNINIT`] which is just an inversion of
/// [`IS_INIT`], and therefore always correct.
///
/// [`IS_INIT`]: InitStatus::IS_INIT
/// [`IS_UNINIT`]: InitStatus::IS_UNINIT
pub unsafe trait InitStatus {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
}

// SAFETY: `Uninit` represents uninitialised
unsafe impl InitStatus for Uninit {
	const IS_INIT: bool = false;
}

// SAFETY: `Init` represents initialised
unsafe impl<T> InitStatus for Init<T>
where
	T: ?Sized
{
	const IS_INIT: bool = true;
}

/// Marker trait for marker structs that represent uninitialised state
///
/// # Safety
///
/// Marker struct must actually represent an uninitialised state.
// ?????
// #[diagnostic::on_unimplemented(
// 	message = "this field has already been initialised"
// )]
pub unsafe trait IsUninit: InitStatus {}

// SAFETY: `Uninit` represents uninitialised
unsafe impl IsUninit for Uninit {}

/// Marker trait for marker structs that represent initialised state
///
/// # Safety
///
/// Marker struct must actually represent an initialised state.
pub unsafe trait IsInit: InitStatus {}

// SAFETY: `Init` represents initialised
unsafe impl<T> IsInit for Init<T>
where
	T: ?Sized
{}

// todo i dont know if this is useful as a generic thing
// /// Types that can be used safely for slots of type `T`
// ///
// /// For example, if we have an API that is expecting a string, we would
// /// implement this trait for string types.
// pub unsafe trait Slot<S>
// where
// 	Self: Sized
// {
// 	type TypeMarker: TypeMarker;
//
// 	/// Output type of reading from a slot previously written to (can be anything)
// 	type Result: Sized;
//
// 	/// Writes `self` into `slot`, doing as little work as needed
// 	///
// 	/// Work should be deferred to [`read`], if possible.
// 	///
// 	/// # Safety
// 	///
// 	/// Implementors must store a value into `slot`, that [`read`]
// 	/// can read back later. [`read`] is allowed to rely on the fact that
// 	/// something has been written.
// 	///
// 	/// It may be undesireable to call [`write`] twice on the same `slot`,
// 	/// but even with that, there is no _strict_ requirement that callers only
// 	/// call [`write`] once.
// 	///
// 	/// [`read`]: Slot::read
// 	/// [`write`]: Slot::write
// 	unsafe fn write(self, slot: &mut S);
//
// 	/// Reads back what was written to `slot` in [`write`], then processes it
// 	/// into the read output type as necessary
// 	///
// 	/// # Safety
// 	///
// 	/// Implementors can assume that [`write`] has been called on `slot` and a
// 	/// value has been written as expected.
// 	///
// 	/// Callers must call [`write`] on this slot first, before passing it
// 	/// to this function.
// 	///
// 	/// [`write`]: Slot::write
// 	unsafe fn read(slot: S) -> Self::Result;
//
// 	/// Converts the read output type into a reference of type
// 	/// [`&ExternAny`](crate::ExternAny)
// 	///
// 	/// This function is used so implementors can return non reference types.
// 	/// Trait consumers would then take ownership of the provided output, and
// 	/// use this function to get a reference type [`&ExternAny`](crate::ExternAny)
// 	/// from it.
// 	fn as_ref(result: &Self::Result) -> &crate::ExternAny;
// }

#[macro_export]
macro_rules! gen_builder_state {
	{
		$(
			$(#[$state_meta:meta])*
			state
		)?

		$(
			$(#[$container_meta:meta])*
			container
		)?

		$(
			$(#[$uninit_meta:meta])*
			uninit
		)?

		$(
			field $field:ident;
			init $field_init:ident;
		)*
	} => {
		$($(#[$state_meta])*)?
		pub trait State {
			$(
				type $field: InitStatus;
				type $field_init<T: ?Sized, M: ?Sized + TypeMarker>: State;
			)*
		}

		#[allow(
			unused_parens,
			reason = "automatically generated"
		)]
		#[expect(
			clippy::allow_attributes,
			reason = "automatically generated (lint might not actually trigger, depending on input)"
		)]
		$($(#[$container_meta])*)?
		pub struct StateContainer<$($field),*> {
			__marker: PhantomDataInvariant<(
				$($field),*
			)>
		}

		gen_state! {
			@impl gen_uninit
			$(
				$(#[$uninit_meta])*
				uninit
			)?

			{}
			{ $($field)* }
		}

		impl<$(
			$field: InitStatus
		),*> State for StateContainer<$($field),*> {
			gen_state! {
				@impl state_init_types
				{}
				{}
				{ $($field $field_init)* }
			}
		}
	};

	{
		@impl gen_uninit
		$(
			$(#[$uninit_meta:meta])*
			uninit
		)?

		{ $($uninit_type:ident)* }
		{
			$field:ident
			$($field_rest:ident)*
		}
	} => {
		gen_state! {
			@impl gen_uninit
			$(
				$(#[$uninit_meta])*
				uninit
			)?

			{
				$($uninit_type)*
				Uninit
			}
			{ $($field_rest)* }
		}
	};

	{
		@impl gen_uninit
		$(
			$(#[$uninit_meta:meta])*
			uninit
		)?

		{ $($uninit_type:ident)* }
		{}
	} => {
		pub type StateUninit = StateContainer<
			$($uninit_type),*
		>;
	};

	{
		@impl state_init_types
		{}
		{}
		{}
	} => {};

	{
		@impl state_init_types
		{}
		{}
		{
			$field_next:ident $field_init_next:ident
			$($field_rest:ident $field_init_rest:ident)*
		}
	} => {
		gen_state! {
			@impl state_init_types
			{}
			{ $field_next $field_init_next }
			{ $($field_rest $field_init_rest)* }
		}
	};

	{
		@impl state_init_types
		{ $($field_prev:ident $field_init_prev:ident)* }
		{ $field:ident $field_init:ident }
		{
			$field_next:ident $field_init_next:ident
			$($field_rest:ident $field_init_rest:ident)*
		}
	} => {
		type $field = $field;
		type $field_init<T: ?Sized, M: ?Sized + TypeMarker> = StateContainer<
			$($field_prev,)*
			Init<T, M>,
			$field_next,
			$($field_rest),*
		>;

		gen_state! {
			@impl state_init_types
			{
				$($field_prev $field_init_prev)*
				$field $field_init
			}
			{ $field_next $field_init_next }
			{ $($field_rest $field_init_rest)* }
		}
	};

	{
		@impl state_init_types
		{ $($field_prev:ident $field_init_prev:ident)* }
		{ $field:ident $field_init:ident }
		{}
	} => {
		type $field = $field;
		type $field_init<T: ?Sized, M: ?Sized + TypeMarker> = StateContainer<
			$($field_prev,)*
			Init<T, M>
		>;
	};
}
pub use gen_builder_state;

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;
