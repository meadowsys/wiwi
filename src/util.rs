//! API utilities and thingies
//!
//! The items that are publicly visible in this module can be considered stable.
//! Anything that is internal only won't be visible.

// Checklist™ v2
// - use statements
// - builder struct (with repr(transparent), `inner`, `__marker`)
// - builder inner struct
// - `gen_state!` invocation (generates state trait, state
//   container struct, uninit type def, impl state for statecontainer)
// - impl builder uninit
// - impl blocks for finished ones with `call_fn` or `build` fns
// - impl block for builder fns, `change_state`, internal functions etc
// - target slot union definitions (will want `uninit`, likely will
//   want `any`, and whatever other incompatible types in there), and
//   associated impls (`Slot` and `SlotUnchecked` impls etc)

pub use core::marker::PhantomData;

pub type PhantomInvariant<T> = PhantomData<fn(T) -> T>;

/// Marker struct for a field in the uninitialised state
pub struct Uninit {
	__private: ()
}

/// Marker struct for a field in the initialised state, optionally
/// holding more state in the form of another type `T`
pub struct Init<T = ()>
where
	T: ?Sized
{
	__marker: PhantomInvariant<T>
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
#[diagnostic::on_unimplemented(
	message = "this field has already been initialised"
)]
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

/// Types that can be used safely for slots of type `T`, just like
/// [`SlotUnchecked`], but can only be implemented on types for which it is
/// correct to use
///
/// For example, if we have an API that is expecting a string, we would only
/// implement this trait for string types, but we can still implement
/// [`SlotUnchecked`] for types that isn't guaranteed to be a string,
/// like [`ExternAny`](crate::ExternAny).
///
/// To implement this trait, implement [`SlotUnchecked`] first, as that trait
/// contains the actual implementation functionality. This trait is "only"
/// a marker trait for those who implement [`SlotUnchecked`], and can guarantee
/// that all values of the type are valid for this slot.
pub unsafe trait Slot<T>
where
	Self: SlotUnchecked<T>
{}

/// Types that can be used for slots of type `T`, just like [`Slot`], but much,
/// _much_ looser in restrictions in which types can implement this trait
///
/// See documentation on [`Slot`] for more details on these two traits.
pub unsafe trait SlotUnchecked<T>
where
	Self: Sized
{
	/// Output type of reading from a slot previously written to (can be anything)
	type Result: Sized;

	/// Writes `self` into `slot`, doing as little work as needed
	///
	/// Work should be deferred to [`read`], if possible.
	///
	/// # Safety
	///
	/// Implementors must store a value into `slot`, that [`read`]
	/// can read back later. [`read`] is allowed to rely on the fact that
	/// something has been written.
	///
	/// It may be undesireable to call [`write`] twice on the same `slot`,
	/// but even with that, there is no _strict_ requirement that callers only
	/// call [`write`] once.
	///
	/// [`read`]: SlotUnchecked::read
	/// [`write`]: SlotUnchecked::write
	unsafe fn write(self, slot: &mut T);

	/// Reads back what was written to `slot` in [`write`], then processes it
	/// into the read output type as necessary
	///
	/// # Safety
	///
	/// Implementors can assume that [`write`] has been called on `slot` and a
	/// value has been written as expected.
	///
	/// Callers must call [`write`] on this slot first, before passing it
	/// to this function.
	///
	/// [`write`]: SlotUnchecked::write
	unsafe fn read(slot: T) -> Self::Result;

	/// Converts the read output type into a reference of type
	/// [`&ExternAny`](crate::ExternAny)
	///
	/// This function is used so implementors can return non reference types.
	/// Trait consumers would then take ownership of the provided output, and
	/// use this function to get a reference type [`&ExternAny`](crate::ExternAny)
	/// from it.
	fn as_ref(result: &Self::Result) -> &crate::ExternAny;
}

macro_rules! gen_builder {
	{
		$(#[$struct_meta:meta])*
		$struct_name:ident {
			deref: $deref_ty:ty;

			$(
				$field_name:ident {
					ty: $field_ident:ident;
					ty_init: $field_ident_init:ident;
					ty_slot: $field_ident_slot:ident;

					$(slot: $slot_type:ty;)*

					$(slot_impl $($unsafe:ident)?: $slot_impl_ty:ident {
						$($slot_impl_body:tt)*
					})*
				}
			)*
		}
	} => {
		#[repr(transparent)]
		$(#[$struct_meta])*
		pub struct $struct_name<S>
		where
			S: State
		{
			inner: Inner,
			__marker: PhantomData<S>
		}

		// - todo builder inner struct
		struct Inner {
			$($field_name: $field_ident_slot),*
		}

		// - todo state trait
		pub trait State {
			$(
				type $field_ident: InitStatus;
				type $field_ident_init<'h, T>: State;
			)*
		}

		// - todo state container
		// - todo uninit type def
		// - todo impl state for state container

		// - todo impl builder uninit

		// - todo impl blocks for finished ones with `call_fn` or `build` fns

		// - todo impl block for builder fns, `change_state`, internal functions etc

		// - todo target slot union definitions (will want `uninit`, likely will
		//   want `any`, and whatever other incompatible types in there), and
		//   associated impls (`Slot` and `SlotUnchecked` impls etc)

		$(
			pub union $field_ident_slot {
				uninit: ()
			}
		)*
	};
}
pub(crate) use gen_builder;
