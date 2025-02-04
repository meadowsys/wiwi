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
/// [`IS_INIT`](InitStatus::IS_INIT) and [`IS_UNINIT`](InitStatus::IS_UNINIT)
/// must both be set correctly to acccurately represent the state of the field.
pub unsafe trait InitStatus {
	const IS_INIT: bool;
	const IS_UNINIT: bool = !Self::IS_INIT;
}

unsafe impl InitStatus for Uninit {
	const IS_INIT: bool = false;
}

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
pub unsafe trait IsUninit: InitStatus {}

unsafe impl IsUninit for Uninit {}

/// Marker trait for marker structs that represent initialised state
///
/// # Safety
///
/// Marker struct must actually represent an initialised state.
pub unsafe trait IsInit: InitStatus {}

unsafe impl<T> IsInit for Init<T>
where
	T: ?Sized
{}

pub unsafe trait AcceptableInSlot<T> {
	type Result: Sized;
	unsafe fn write(self, slot: &mut T);
	unsafe fn read(slot: T) -> Self::Result;
}

pub unsafe trait AcceptableInSlotUnchecked<T> {
	type Result: Sized;
	unsafe fn write_unchecked(self, slot: &mut T);
	unsafe fn read_unchecked(slot: T) -> Self::Result;
}

unsafe impl<T, T2> AcceptableInSlotUnchecked<T2> for T
where
	T: AcceptableInSlot<T2>
{
	type Result = <Self as AcceptableInSlot<T2>>::Result;

	#[inline]
	unsafe fn write_unchecked(self, slot: &mut T2) {
		unsafe { self.write(slot) }
	}

	#[inline]
	unsafe fn read_unchecked(slot: T2) -> Self::Result {
		unsafe { T::read(slot) }
	}
}

pub(crate) type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;

pub(crate) trait PtrWriteCastLifetimeExt<T> {
	/// Convenience method to change the lifetime of the reference type of the
	/// pointer, then call `write` on the casted pointer
	///
	/// # Safety
	///
	/// You must ensure that your lifetimes are correct, as well as follow
	/// safety requirements of [`ptr::write`](std::ptr::write).
	unsafe fn cast_lifetime_write(self, value: &T);
}

impl<T> PtrWriteCastLifetimeExt<T> for *mut &T {
	#[inline]
	unsafe fn cast_lifetime_write(self, value: &T) {
		unsafe { self.cast::<&T>().write(value) }
	}
}

/// macro for the boilerplate of `let value = unsafe { Value::read(self.inner.value) };`
///
/// # Examples
///
/// ```ignore
/// read_slots! {
///    self
///    value: Value
///    value2: Value2
///    cheese: Cheese
/// }
/// ```
///
/// Expands to:
///
/// ```ignore
/// let value = unsafe { Value::read(self.inner.value) };
/// let value2 = unsafe { Value2::read(self.inner.value2) };
/// let cheese = unsafe { Cheese::read(self.inner.cheese) };
/// ```
macro_rules! read_slots {
	{
		$self:ident
		$($ident:ident: $ty:ident)*
	} => {
		$(
			let $ident = unsafe { $ty::read($self.inner.$ident) };
			let $ident = $ident.as_ref();
		)*
	}
}
pub(crate) use read_slots;

macro_rules! gen_state {
	{
		$state:ident
		$state_container:ident
		$state_uninit:ident
		$(
			$field:ident
			$field_init:ident
			$field_init_with:ident
		)*
	} => {
		pub trait $state {
			$(
				type $field: $crate::builder::InitStatus;
				type $field_init: $state;
				type $field_init_with<S: ?Sized>: $state;
			)*
		}

		#[allow(
			unused_parens,
			reason = "automatically generated"
		)]
		pub struct $state_container<$($field),*> {
			__marker: $crate::prelude_internal::PhantomDataInvariant<(
				$($field),*
			)>
		}

		$crate::builder::gen_state! {
			@impl gen_uninit
			$state_container
			$state_uninit
			{}
			{ $($field)* }
		}

		impl<$(
			$field: $crate::builder::InitStatus
		),*> $state for $state_container<$($field),*> {
			$crate::builder::gen_state! {
				@impl state_init_types
				$state_container
				{}
				{}
				{ $(
					$field
					$field_init
					$field_init_with
				)* }
			}
		}
	};

	{
		@impl state_init_types
		$state_container:ident
		{}
		{}
		{
			$field_next:ident
			$field_init_next:ident
			$field_init_with_next:ident
			$(
				$field_rest:ident
				$field_init_rest:ident
				$field_init_with_rest:ident
			)*
		}
	} => {
		$crate::builder::gen_state! {
			@impl state_init_types
			$state_container
			{}
			{
				$field_next
				$field_init_next
				$field_init_with_next
			}
			{ $(
				$field_rest
				$field_init_rest
				$field_init_with_rest
			)* }
		}
	};

	{
		@impl state_init_types
		$state_container:ident
		{ $(
			$field_prev:ident
			$field_init_prev:ident
			$field_init_with_prev:ident
		)* }
		{
			$field:ident
			$field_init:ident
			$field_init_with:ident
		}
		{
			$field_next:ident
			$field_init_next:ident
			$field_init_with_next:ident
			$(
				$field_rest:ident
				$field_init_rest:ident
				$field_init_with_rest:ident
			)*
		}
	} => {
		type $field = $field;
		type $field_init = $state_container<
			$($field_prev,)*
			$crate::builder::Init,
			$field_next,
			$($field_rest,)*
		>;
		type $field_init_with<S: ?Sized> = $state_container<
			$($field_prev,)*
			crate::builder::Init<S>,
			$field_next,
			$($field_rest,)*
		>;

		$crate::builder::gen_state! {
			@impl state_init_types
			$state_container
			{
				$(
					$field_prev
					$field_init_prev
					$field_init_with_prev
				)*

				$field
				$field_init
				$field_init_with
			}
			{
				$field_next
				$field_init_next
				$field_init_with_next
			}
			{ $(
				$field_rest
				$field_init_rest
				$field_init_with_rest
			)* }
		}
	};

	{
		@impl state_init_types
		$state_container:ident
		{ $(
			$field_prev:ident
			$field_init_prev:ident
			$field_init_with_prev:ident
		)* }
		{
			$field:ident
			$field_init:ident
			$field_init_with:ident
		}
		{}
	} => {
		type $field = $field;
		type $field_init = $state_container<
			$($field_prev,)*
			$crate::builder::Init
		>;
		type $field_init_with<S: ?Sized> = $state_container<
			$($field_prev,)*
			crate::builder::Init<S>,
		>;
	};

	{
		@impl gen_uninit
		$state_container:ident
		$state_uninit:ident
		{ $(($($uninit_ty:tt)*))* }
		{
			$field:ident
			$($field_rest:ident)*
		}
	} => {
		$crate::builder::gen_state! {
			@impl gen_uninit
			$state_container
			$state_uninit
			{
				$(($($uninit_ty)*))*
				(crate::builder::Uninit)
			}
			{ $($field_rest)* }
		}
	};

	{
		@impl gen_uninit
		$state_container:ident
		$state_uninit:ident
		{ $(($($uninit_ty:tt)*))* }
		{}
	} => {
		pub type $state_uninit = $state_container<
			$($($uninit_ty)*),*
		>;
	};
}
pub(crate) use gen_state;
