//! Common builder API utilities and stuffs

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

pub type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;

pub trait PtrWriteCastLifetimeExt<T> {
	/// Convenience method to cast the pointer, then call `write` on the casted pointer
	///
	/// # Safety
	///
	/// You must ensure the type the pointer is cast to is a valid cast, and follow
	/// safety requirements of [`ptr::write`](std::ptr::write).
	unsafe fn cast_lifetime_write<'h2>(self, value: &'h2 T);
}

impl<'h, T> PtrWriteCastLifetimeExt<T> for *mut &'h T {
	#[inline(always)]
	unsafe fn cast_lifetime_write<'h2>(self, value: &'h2 T) {
		unsafe { self.cast::<&'h2 T>().write(value) }
	}
}

/// macro for the boilerplate of `let value = unsafe { self.inner.value.assume_init() };`
///
/// # Examples
///
/// ```no_run
/// unsafe_assume_init! { self value value2 cheese }
/// ```
///
/// Expands to:
///
/// ```no_run
/// let value = unsafe { self.inner.value.assume_init() };
/// let value2 = unsafe { self.inner.value2.assume_init() };
/// let cheese = unsafe { self.inner.cheese.assume_init() };
/// ```
macro_rules! unsafe_assume_init {
	{ $self:ident $($ident:ident)* } => {
		$(
			let $ident = unsafe {
				$self.inner.$ident.assume_init()
			};
		)*
	}
}
pub(crate) use unsafe_assume_init;
