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

/// Marker struct for a field in the uninitialised state
pub struct Uninit {
	__private: ()
}

/// Marker struct for a field in the initialised state, optionally
/// containing more state in the form of another type `T`
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

/// [`PhantomData`] but invariant over `T`
///
/// To construct values of this type, you still must use the expression
/// `PhantomData`, as Rust doesn't like using type definitions as unit struct
/// constructors.
pub(crate) type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;

/// Provides convenience method
/// [`cast_lifetime_write`](PtrWriteCastLifetimeExt::cast_lifetime_write)
/// for pointers to references, to cast the lifetime of the reference type to
/// something else before writing to the pointer (extremely, _extremely_
/// simple to misuse!)
pub(crate) trait PtrWriteCastLifetimeExt<T> {
	/// Convenience method to change the lifetime of the reference type of the
	/// pointer, then call `write` on the casted pointer
	///
	/// # Safety
	///
	/// You must ensure that your lifetimes are correct, as well as follow
	/// safety requirements of [`ptr::write`](core::ptr::write).
	unsafe fn cast_lifetime_write(self, value: &T);
}

impl<T> PtrWriteCastLifetimeExt<T> for *mut &T {
	#[inline(always)]
	unsafe fn cast_lifetime_write(self, value: &T) {
		unsafe { self.cast::<&T>().write(value) }
	}
}

macro_rules! gen_builder {
	{
		$(#[$meta:meta])*
		$name:ident

		$($field:ident: $slot:ident)*
	} => {
		#[repr(transparent)]
		$(#[$meta])*
		pub struct $name<'h, S>
		where
			S: State
		{
			inner: Inner<'h>,
			__marker: PhantomDataInvariant<S>
		}

		struct Inner<'h> {
			$($field: $slot<'h>),*
		}

		impl $name<'static, StateUninit> {
			#[inline(always)]
			pub(super) fn new() -> Self {
				Self {
					inner: Inner {
						$($field: $slot { uninit: () }),*
						// todo when all slots have been converted to use the macro, use this
						// $($field: $slot::uninit()),*
					},
					__marker: PhantomData
				}
			}
		}
	}
}
pub(crate) use gen_builder;

macro_rules! gen_state {
	{
		$state:ident
		$state_container:ident
		$state_uninit:ident
		$($field:ident $field_init:ident)*
	} => {
		pub trait $state {
			$(
				type $field: InitStatus;
				type $field_init<S: ?Sized>: $state;
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

		gen_state! {
			@impl gen_uninit
			$state_container
			$state_uninit
			{}
			{ $($field)* }
		}

		impl<$(
			$field: InitStatus
		),*> $state for $state_container<$($field),*> {
			gen_state! {
				@impl state_init_types
				$state_container
				{}
				{}
				{ $($field $field_init)* }
			}
		}
	};

	{
		@impl state_init_types
		$state_container:ident
		{}
		{}
		{
			$field_next:ident $field_init_next:ident
			$($field_rest:ident $field_init_rest:ident)*
		}
	} => {
		gen_state! {
			@impl state_init_types
			$state_container
			{}
			{ $field_next $field_init_next }
			{ $($field_rest $field_init_rest)* }
		}
	};

	{
		@impl state_init_types
		$state_container:ident
		{ $($field_prev:ident $field_init_prev:ident)* }
		{ $field:ident $field_init:ident }
		{
			$field_next:ident $field_init_next:ident
			$($field_rest:ident $field_init_rest:ident)*
		}
	} => {
		type $field = $field;
		type $field_init<S: ?Sized> = $state_container<
			$($field_prev,)*
			Init<S>,
			$field_next,
			$($field_rest,)*
		>;

		gen_state! {
			@impl state_init_types
			$state_container
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
		$state_container:ident
		{ $($field_prev:ident $field_init_prev:ident)* }
		{ $field:ident $field_init:ident }
		{}
	} => {
		type $field = $field;
		type $field_init<S: ?Sized> = $state_container<
			$($field_prev,)*
			Init<S>,
		>;
	};

	{
		@impl gen_uninit
		$state_container:ident
		$state_uninit:ident
		{ $($uninit_ty:ident)* }
		{
			$field:ident
			$($field_rest:ident)*
		}
	} => {
		gen_state! {
			@impl gen_uninit
			$state_container
			$state_uninit
			{
				$($uninit_ty)*
				Uninit
			}
			{ $($field_rest)* }
		}
	};

	{
		@impl gen_uninit
		$state_container:ident
		$state_uninit:ident
		{ $($uninit_ty:ident)* }
		{}
	} => {
		pub type $state_uninit = $state_container<
			$($uninit_ty),*
		>;
	};
}
pub(crate) use gen_state;

macro_rules! gen_change_state {
	($name:ident) => {
		#[inline(always)]
		unsafe fn change_state<'h2, S2, F>(self, f: F) -> $name<'h2, S2>
		where
			'h: 'h2,
			S2: State,
			F: FnOnce(&mut $name<'h2, S2>)
		{
			let mut changed = $name {
				inner: self.inner,
				__marker: PhantomData
			};

			f(&mut changed);
			changed
		}
	};
}
pub(crate) use gen_change_state;

macro_rules! gen_builder_fn {
	{
		$name:ident

		$field_state:ident
		$field_state_init:ident
		$slot:ident

		$(#[$meta:meta])*
		$field:ident

		$(#[$meta_unchecked:meta])*
		$fn_name_unchecked:ident
	} => {
		#[inline(always)]
		$(#[$meta])*
		pub fn $field<'h2, T>(
			self,
			$field: T
		) -> $name<'h2, S::$field_state_init<T>>
		where
			'h: 'h2,
			S::$field_state: IsUninit,
			T: Slot<$slot<'h2>>
		{
			unsafe { self.$fn_name_unchecked($field) }
		}

		#[doc = concat!(
			"Setter for [`",
			stringify!($field),
			"`](Self::",
			stringify!($field),
			") with much, _much_ looser type restrictions"
		)]
		#[doc = ""]
		#[doc = concat!(
			"See the safer setter ([`",
			stringify!($field),
			"`](Self::",
			stringify!($field),
			")) for more information."
		)]
		#[inline(always)]
		$(#[$meta_unchecked])*
		pub unsafe fn $fn_name_unchecked<'h2, T>(
			self,
			$field: T
		) -> $name<'h2, S::$field_state_init<T>>
		where
			'h: 'h2,
			S::$field_state: IsUninit,
			T: SlotUnchecked<$slot<'h2>>
		{
			unsafe { self.change_state(|b| $field.write(&mut b.inner.$field)) }
		}
	};
}
pub(crate) use gen_builder_fn;

/// macro for the boilerplate of calling `SlotUnchecked::read`
/// followed by conversion to `&JsValue`
///
/// # Examples
///
/// ```ignore
/// unsafe {
///    read_slots! {
///       self
///       value: Value
///       value2: Value2
///       cheese: Cheese
///    }
/// }
/// ```
///
/// Expands to:
///
/// ```ignore
/// unsafe {
///    let value = Value::read(self.inner.value);
///    let value = value.as_ref().as_js_value();
///    let value2 = Value2::read(self.inner.value2);
///    let value2 = value2.as_ref().as_js_value();
///    let cheese = Cheese::read(self.inner.cheese);
///    let cheese = cheese.as_ref().as_js_value();
/// }
/// ```
///
/// Well... not quite, but, good enough for purposes of demonstration.
macro_rules! read_slots {
	{
		$self:ident
		$($ident:ident: $ty:ident)*
	} => {
		$(
			let $ident = $ty::read($self.inner.$ident);
			let $ident = $ty::as_ref(&$ident).as_js_value();
		)*
	}
}
pub(crate) use read_slots;

macro_rules! gen_slot {
	{
		$(#[$union_meta:meta])*
		$slot:ident
		$(field $field:ident { $($field_type:tt)* })*

		$(
			impl for { $($impl_type:tt)* } $($unsafe:ident)? {
				$($impl_stuff:tt)*
			}
		)*
	} => {
		$(#[$union_meta])*
		pub union $slot<'h> {
			uninit: (),
			$($field: $($field_type)*),*
		}

		impl $slot<'static> {
			#[inline(always)]
			pub(crate) fn uninit() -> Self {
				Self { uninit: () }
			}
		}

		gen_slot! {
			@impl trait_impl
			unsafe $slot { &'h ExternAny } {
				result { &'h ExternAny }
				write(self, slot) {
					*slot = $slot { any: self }
				}
				read(slot) {
					unsafe { slot.any }
				}
				as_ref(result) {
					result
				}
			}
		}

		$(
			gen_slot! {
				@impl trait_impl
				$($unsafe)? $slot { $($impl_type)* } { $($impl_stuff)* }
			}
		)*
	};

	{
		@impl trait_impl
		unsafe $slot:ident { $($impl_type:tt)* } {
			result { $($result:tt)* }

			write($self:ident, $slot_write_param:ident) $(-> { $($write_return_type:tt)* })? {
				$($write_impl:tt)*
			}

			read($slot_read_param:ident) $(-> { $($read_return_type:tt)* })? {
				$($read_impl:tt)*
			}

			as_ref($result_as_ref_param:ident) $(-> { $($as_ref_return_type:tt)* })? {
				$($as_ref_impl:tt)*
			}
		}
	} => {
		unsafe impl<'h> SlotUnchecked<$slot<'h>> for $($impl_type)* {
			type Result = $($result)*;

			#[inline]
			unsafe fn write(
				$self,
				$slot_write_param: &mut $slot<'h>
			) {
				$($write_impl)*
			}

			#[inline]
			unsafe fn read(
				$slot_read_param: $slot<'h>
			) -> $($result)* {
				$($read_impl)*
			}

			#[inline]
			fn as_ref<'h2>(
				$result_as_ref_param: &'h2 $($result)*
			) -> gen_slot! {
				@impl mk_return_type_as_ref
				$($as_ref_return_type)*
			} {
				$($as_ref_impl)*
			}
		}
	};

	{
		@impl trait_impl
		$slot:ident { $($impl_type:tt)* } {
			$($stuff:tt)*
		}
	} => {
		unsafe impl<'h> Slot<$slot<'h>> for $($impl_type)* {}

		gen_slot! {
			@impl trait_impl
			unsafe $slot { $($impl_type)* } {
				$($stuff)*
			}
		}
	};

	{ @impl mk_return_type_as_ref } => { &'h2 ExternAny };
	{ @impl mk_return_type_as_ref $($return:tt)* } => { $($return)* };
}
pub(crate) use gen_slot;
