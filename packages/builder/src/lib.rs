pub struct Uninit {
	__private: ()
}

pub struct Init<T = (), M = ()>
where
	T: ?Sized,
	M: ?Sized + Marker
{
	__marker_t: PhantomDataInvariant<T>,
	__marker_m: PhantomDataInvariant<M>,
}

pub trait Marker {}

impl Marker for () {}

/// # Safety
///
/// [`IS_INIT`] and [`IS_UNINIT`] must both be set correctly to acccurately
/// represent the state of the field.
pub unsafe trait InitStatus {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
}

// SAFETY: `Uninit` represents uninitialised state
unsafe impl InitStatus for Uninit {
	const IS_INIT: bool = false;
}

// SAFETY: `Init` represents initialised state
unsafe impl<T, M> InitStatus for Init<T, M>
where
	T: ?Sized,
	M: ?Sized + Marker
{
	const IS_INIT: bool = true;
}

/// # Safety
///
/// Marker struct must actually represent an uninitialised state.
pub unsafe trait IsUninit: InitStatus {}

// SAFETY: `Uninit` represents uninitialised state
unsafe impl IsUninit for Uninit {}

/// # Safety
///
/// Marker struct must actually represent an initialised state.
pub unsafe trait IsInit: InitStatus {}

// SAFETY: `Init` represents initialised state
unsafe impl<T, M> IsInit for Init<T, M>
where
	T: ?Sized,
	M: ?Sized + Marker
{}

type PhantomDataInvariant<T> = core::marker::PhantomData<fn(T) -> T>;
