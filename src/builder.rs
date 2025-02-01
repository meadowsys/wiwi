//! Common builder API stuffs

// Checklist™
// - use statements
// - builder struct (with repr(transparent), `inner`, `__marker`)
// - builder inner struct
// - state trait
// - state container struct
// - type definitions for important states
// - impl state for statecontainer
// - impl builder uninit
// - impl blocks for finished ones with `call` or `build` fns
// - impl block for builder fns, `change_state`, internal functions etc

pub use std::marker::PhantomData;

pub struct Uninit {
	__private: ()
}

pub struct Init {
	__private: ()
}

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

/// Marker trait for marker structs that represent uninitialised state
///
/// # Safety
///
/// Marker struct must actually represent an uninitialised state.
pub unsafe trait IsUninit: InitStatus {}

unsafe impl IsUninit for Uninit {}

/// Marker trait for marker structs that represent initialised state
///
/// # Safety
///
/// Marker struct must actually represent an initialised state.
pub unsafe trait IsInit: InitStatus {}

unsafe impl IsInit for Init {}

/// Items that can act as initialisers for a specific field marked by `F`
///
/// # Safety
///
/// Implementations of this trait [`Initialiser<F, S>`] must be paired with a
/// correct implementation of [`InitState<F, S>`] as well, and the corresponding
/// [store] and [retrieve] functions must store and retrieve from the same location
/// in `slot`, for memory safety.
pub unsafe trait Initialiser<F, S>: Sized {
	/// Type to use to mark the initialised state
	type Init: InitState<Self, F, S, Result = Self::Result>;

	/// End result of retrieving the value and processing it,
	/// usually bound by some trait
	type Result;

	/// Store `self` in the given slot
	///
	/// # Safety
	///
	/// The item stored must be read from the same spot
	/// by the corresponding [`InitState`] impl.
	unsafe fn store_in_slot(self, slot: &mut S);
}

/// Type state marking initialised state for a [`Initialiser`]
///
/// # Safety
///
/// See safety section on [`Initialiser`].
pub unsafe trait InitState<T, F, S>: Sized + IsInit {
	/// End result of retrieving the value and processing it,
	/// usually bound by some trait
	type Result;

	/// Retrieve the item from the given slot, and process it to [`Result`]
	///
	/// # Safety
	///
	/// The item read must be the same spot that the
	/// corresponding [`Initialiser`] impl stored to.
	unsafe fn retrieve_from_slot(slot: S) -> Self::Result;
}

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;
