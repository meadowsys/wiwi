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
//   associated impls (`Slot` impls etc)

pub use self::marker::*;
pub use core::marker::PhantomData;

pub mod marker;

/// Marker struct for a field in the uninitialised state
pub struct Uninit {
	__private: ()
}

/// Marker struct for a field in the initialised state, optionally
/// containing more state in the form of another type `S`, and a more
/// "general" type that can be "matched upon" in implementations, in `T`
pub struct Init<T = (), M = ()>
where
	T: ?Sized,
	M: ?Sized + TypeMarker
{
	// need two seperate markers because `T` and `M` are `?Sized`
	__marker_t: PhantomDataInvariant<T>,
	__marker_g: PhantomDataInvariant<M>
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
unsafe impl<T, M> InitStatus for Init<T, M>
where
	T: ?Sized,
	M: ?Sized + TypeMarker
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

/// Types that can be used safely for slots of type `T`
///
/// For example, if we have an API that is expecting a string, we would
/// implement this trait for string types.
pub unsafe trait Slot<S>
where
	Self: Sized
{
	type TypeMarker: TypeMarker;

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
	/// [`read`]: Slot::read
	/// [`write`]: Slot::write
	unsafe fn write(self, slot: &mut S);

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
	/// [`write`]: Slot::write
	unsafe fn read(slot: S) -> Self::Result;

	/// Converts the read output type into a reference of type
	/// [`&ExternAny`](crate::ExternAny)
	///
	/// This function is used so implementors can return non reference types.
	/// Trait consumers would then take ownership of the provided output, and
	/// use this function to get a reference type [`&ExternAny`](crate::ExternAny)
	/// from it.
	fn as_ref(result: &Self::Result) -> &crate::ExternAny;
}

/// [`PhantomData`] but invariant over `T` and marks the type as `!Send` and `!Sync`.
///
/// To construct values of this type, you still must use the expression
/// `PhantomData`, as Rust doesn't like using type definitions as unit struct
/// constructors.
pub(crate) type PhantomDataBuilder<S> = PhantomData<(
	// `S` invariant
	fn(S) -> S,
	// `!Send` and `!Sync`
	*mut ()
)>;

/// [`PhantomData`] but invariant over `T`
///
/// To construct values of this type, you still must use the expression
/// `PhantomData`, as Rust doesn't like using type definitions as unit struct
/// constructors.
pub(crate) type PhantomDataInvariant<T> = PhantomData<fn(T) -> T>;

macro_rules! gen_struct {
	{
		$(#[$meta:meta])*
		// name of "builder" struct
		struct $struct_name_without_state:ident $struct_name:ident;

		// generates deref impl if this is present
		$(
			deref $deref_type:ty;
			// uses this expression to initialise the value on first deref
			deref_value $deref_value:expr;
		)?

		// generates a different deref impl if this is present
		// only this or the one above can be present, of course
		$(untracked_deref $untracked_field_deref:ident: $untracked_field_deref_type:ty;)?

		$(
			field
			// field name
			$field:ident
			// name of generated slot type (without generics etc)
			$slot:ident;
		)*

		$(untracked_field $untracked_field:ident: $untracked_field_type:ty;)*
	} => {
		pub type $struct_name_without_state = $struct_name<'static, StateUninit>;

		#[repr(transparent)]
		pub struct $struct_name<'h, S: State> {
			inner: Inner<'h>,
			__marker: PhantomDataBuilder<S>
		}

		struct Inner<'h> {
			__use_h: PhantomData<&'h ()>,
			$(__deref: ::core::cell::UnsafeCell<::core::option::Option<$deref_type>>,)?
			$($field: $slot<'h>,)*
			$($untracked_field: $untracked_field_type),*
		}

		impl $struct_name<'static, StateUninit> {
			/// # Safety
			///
			/// This is an autogenerated function that should only be usedd internally.
			/// It might have unknown preconditions on the values that can be passed
			/// in. Because of this, it has been conservatively marked unsafe.
			#[inline(always)]
			const unsafe fn __new(
				$($untracked_field: $untracked_field_type),*
			) -> Self {
				Self {
					inner: Inner {
						__use_h: PhantomData,
						// its `None::<$deref_type>` and not just `None` so rust knows
						// to base the presence of this field on it
						$(__deref: ::core::cell::UnsafeCell::new(None::<$deref_type>),)?
						$($field: $slot::uninit(),)*
						$($untracked_field),*
					},
					__marker: PhantomData
				}
			}
		}

		$(
			impl<'h, S> ::core::ops::Deref for $struct_name<'h, S>
			where
				S: State
			{
				type Target = $deref_type;

				#[inline(always)]
				fn deref(&self) -> &$deref_type {
					// SAFETY: we are not thread safe so taking mut ref like
					// this of the inner value of UnsafeCell temporarily is fine,
					// we won't ever have two mut references (the closure in the call
					// to `get_or_insert_with` can't access `self`)
					unsafe {
						(*self.inner.__deref.get())
							.get_or_insert_with(|| $deref_value)
					}
				}
			}
		)?

		$(
			impl<'h, S> ::core::ops::Deref for $struct_name<'h, S>
			where
				S: State
			{
				type Target = $untracked_field_deref_type;

				#[inline(always)]
				fn deref(&self) -> &$untracked_field_deref_type {
					&self.inner.$untracked_field_deref
				}
			}
		)?
	};
}
pub(crate) use gen_struct;

macro_rules! gen_slot {
	{
		$(#[$meta:meta])*
		// name of slot struct
		slot $slot:ident;
		$(
			field
			// name of field
			$field:ident
			// type of the field to store (can use 'h lifetime in the type)
			$field_type:ty;
		)*
	} => {
		$(#[$meta])*
		pub union $slot<'h> {
			uninit: (),
			any: &'h ExternAny,
			$($field: ::core::mem::ManuallyDrop<$field_type>),*
		}

		impl $slot<'static> {
			#[inline(always)]
			pub(crate) const fn uninit() -> Self {
				// totally not stolen from MaybeUninit uwu
				Self { uninit: () }
			}
		}
	};
}
pub(crate) use gen_slot;

macro_rules! gen_slot_impl {
	// generates slot impl
	{
		$(#[$meta:meta])*
		slot $slot:ident;
		impl$({ $($generics:tt)* })? $impl_type:ty;

		$($stuff:tt)*
	} => {
		unsafe impl<'h $(, $($generics)*)?> Slot<$slot<'h>> for $impl_type {
			gen_slot_impl! {
				@impl
				slot $slot;
				impl $impl_type;

				$($stuff)*
			}
		}
	};

	// Slot::Result
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		result $result_type:ty;

		$($stuff:tt)*
	} => {
		type Result = $result_type;

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	// Slot::TypeMarker = ()
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		no_type_marker;

		$($stuff:tt)*
	} => {
		type TypeMarker = ();

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	// Slot::TypeMarker
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		type_marker $type_marker:ty;

		$($stuff:tt)*
	} => {
		type TypeMarker = $type_marker;

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		no_type_marker;

		$($stuff:tt)*
	} => {
		type TypeMarker = ();

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	// Slot::write
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		write($self:ident, $write_slot:ident) {
			$($write_impl:tt)*
		}

		$($stuff:tt)*
	} => {
		#[inline(always)]
		unsafe fn write($self, $write_slot: &mut $slot<'h>) {
			$($write_impl)*
		}

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	// Slot::read
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		read($read_slot:ident) {
			$($read_impl:tt)*
		}

		$($stuff:tt)*
	} => {
		#[inline(always)]
		unsafe fn read($read_slot: $slot<'h>) -> Self::Result {
			$($read_impl)*
		}

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	// Slot::as_ref
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		as_ref($as_ref_result:ident) {
			$($as_ref_impl:tt)*
		}

		$($stuff:tt)*
	} => {
		#[inline(always)]
		fn as_ref($as_ref_result: &Self::Result) -> &ExternAny {
			$($as_ref_impl)*
		}

		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			$($stuff)*
		}
	};

	// *slot = Slot { field: self }
	// does rely on simple assignment syntax to the field
	// (autoderef if needed etc.)
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		simple_w $field:ident;

		$($stuff:tt)*
	} => {
		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			write(self, slot) {
				*slot = $slot { $field: self }
			}

			$($stuff)*
		}
	};

	// simple_w, additionally just reading the slot field in read
	// unsafe { slot.field }
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		simple_rw $field:ident;

		$($stuff:tt)*
	} => {
		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			simple_w $field;

			read(slot) {
				unsafe { slot.$field }
			}

			$($stuff)*
		}
	};

	// implements Slot::as_ref via just returning result
	// (autoderef if needed etc.)
	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;

		autoderef;

		$($stuff:tt)*
	} => {
		gen_slot_impl! {
			@impl
			slot $slot;
			impl $impl_type;

			as_ref(result) { result }

			$($stuff)*
		}
	};

	{
		@impl
		slot $slot:ident;
		impl $impl_type:ty;
	} => { /* empty uwu */ };
}
pub(crate) use gen_slot_impl;

/// macro for the boilerplate of calling `Slot::read`
/// followed by conversion to `&JsValue`
///
/// # Examples
///
/// ```ignore
/// unsafe {
///    unsafe_read_slots! {
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
///
/// # Safety
///
/// The fields/slots you pass in must actually be valid for reading.
macro_rules! unsafe_read_slots {
	{
		$self:ident
		$($ident:ident: $ty:ident)*
	} => {
		$(
			// SAFETY: caller guarantees this slot is valid to read from
			let $ident = unsafe { $ty::read($self.inner.$ident) };
			let $ident = $ty::as_ref(&$ident).as_js_value();
		)*
	}
}
pub(crate) use unsafe_read_slots;

macro_rules! gen_state {
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
pub(crate) use gen_state;

macro_rules! gen_builder_fns {
	{
		struct $struct_name:ident;

		$(
			state $field_state:ident;
			init $field_init:ident;
			slot $slot:ident;
			$(extra_bounds { $($extra_bounds:tt)* };)?

			$(#[$meta:meta])*
			field $field:ident $($no_lifetime:ident)?;
		)*
	} => {
		impl<'h, S: State> $struct_name<'h, S> {
			gen_change_state!($struct_name);

			$(
				gen_builder_fn! {
					struct $struct_name;

					state $field_state;
					init $field_init;
					slot $slot;
					$(extra_bounds { $($extra_bounds)* };)?

					$(#[$meta:meta])*
					field $field $($no_lifetime)?;
				}
			)*
		}
	}
}
pub(crate) use gen_builder_fns;

macro_rules! gen_builder_fn {
	{
		struct $struct_name:ident;

		state $field_state:ident;
		init $field_init:ident;
		slot $slot:ident;
		$(extra_bounds { $($extra_bounds:tt)* };)?

		$(#[$meta:meta])*
		field $field:ident;
	} => {
		#[inline(always)]
		$(#[$meta])*
		pub fn $field<T>(
			self,
			$field: T
		) -> $struct_name<'h, S::$field_init<T, T::TypeMarker>>
		where
			S::$field_state: IsUninit,
			T: Slot<$slot<'h>>,
			$($($extra_bounds)*)?
		{
			unsafe { self.change_state(|b| $field.write(&mut b.inner.$field)) }
		}
	};
}
pub(crate) use gen_builder_fn;

macro_rules! gen_change_state {
	($struct_name:ident) => {
		#[inline(always)]
		unsafe fn change_state<'h2, S2, F>(self, f: F) -> $struct_name<'h2, S2>
		where
			'h: 'h2,
			S2: State,
			F: FnOnce(&mut $struct_name<'h2, S2>)
		{
			let mut changed = $struct_name {
				inner: self.inner,
				__marker: PhantomData
			};

			f(&mut changed);
			changed
		}
	};
}
pub(crate) use gen_change_state;

macro_rules! gen_call_fn {
	{
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$($rest:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			fn_name call_fn;

			fields {}

			$($rest)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$meta:meta])*
		fn_name $fn_name:ident;

		fields { $($fields:tt)* }

		field $field_name:ident;
		state $field_state:ident;
		init Init<$slot:ident>;

		$($stuff:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			$(#[$meta])*
			fn_name $fn_name;

			fields {
				$($fields)*

				field
				{ $field_state: Slot<$slot<'h>>, }
				{ Init<$field_state> }
				{ $field_name: $field_state }
				{}
			}

			$($stuff)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$meta:meta])*
		fn_name $fn_name:ident;

		fields { $($fields:tt)* }

		field $field_name:ident;
		state $field_state:ident;
		init Init<$slot:ident, $general_type:ty>;

		$($stuff:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			$(#[$meta])*
			fn_name $fn_name;

			fields {
				$($fields)*

				field
				{ $field_state: Slot<$slot<'h>>, }
				{ Init<$field_state, $general_type> }
				{ $field_name: $field_state }
				{}
			}

			$($stuff)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$meta:meta])*
		fn_name $fn_name:ident;

		fields { $($fields:tt)* }

		field $field_name:ident;
		state $field_state:ident;
		init Init<$slot:ident, any $general_type:ident>;

		$($stuff:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			$(#[$meta])*
			fn_name $fn_name;

			fields {
				$($fields)*

				field
				{ $field_state: Slot<$slot<'h>>, $general_type: TypeMarker, }
				{ Init<$field_state, $general_type> }
				{ $field_name: $field_state }
				{}
			}

			$($stuff)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$meta:meta])*
		fn_name $fn_name:ident;

		fields { $($fields:tt)* }

		field $field_name:ident;
		state $field_state:ident;
		init Uninit;

		$($stuff:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			$(#[$meta])*
			fn_name $fn_name;

			fields {
				$($fields)*

				field
				{}
				{ Uninit }
				{}
				{ $field_name }
			}

			$($stuff)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$old_meta:meta])*
		fn_name $old_fn_name:ident;

		fields { $($fields:tt)* }

		$(#[$meta:meta])*
		fn;

		$($stuff:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			$(#[$old_meta])*
			$(#[$meta])*
			fn_name $old_fn_name;

			fields { $($fields)* }

			$($stuff)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$old_meta:meta])*
		fn_name $old_fn_name:ident;

		fields { $($fields:tt)* }

		$(#[$meta:meta])*
		fn $fn_name:ident;

		$($stuff:tt)*
	} => {
		gen_call_fn! {
			@impl nom_fields
			struct $struct_name;
			raw_call $raw_call;
			return $return_type;

			$(#[$old_meta])*
			$(#[$meta])*
			fn_name $fn_name;

			fields { $($fields)* }

			$($stuff)*
		}
	};

	{
		@impl nom_fields
		struct $struct_name:ident;
		raw_call $raw_call:expr;
		return $return_type:ty;

		$(#[$meta:meta])*
		fn_name $fn_name:ident;

		fields {
			$(
				field
				{ $($impl_param:tt)* }
				{ $($struct_param:tt)* }
				{ $($unsafe_read_slots_input:tt)* }
				{ $($unused_fields:tt)* }
			)*
		}
	} => {
		impl<
			'h,
			$($($impl_param)*)*
		> $struct_name<'h, StateContainer<
			$($($struct_param)*),*
		>> {
			#[inline(always)]
			$(#[$meta])*
			pub fn $fn_name(self) -> $return_type {
				// SAFETY: provided slots are valid for reading,
				// enforced by type system
				unsafe_read_slots! {
					self
					$($($unsafe_read_slots_input)*)*
				}

				$($(
					#[allow(
						clippy::drop_non_drop,
						reason = "automatically generated"
					)]
					#[expect(
						clippy::allow_attributes,
						reason = "automatically generated (lint might not actually trigger)"
					)]
					drop(self.inner.$unused_fields);
				)*)*

				$raw_call
			}
		}
	};
}
pub(crate) use gen_call_fn;
