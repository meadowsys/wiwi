//! Utilities for nominal typing

use crate::prelude::*;

/// Declare a new nominal type (alias), with the provided name, a name for the
/// marker type struct, and the wrapped type
///
/// The returned type alias will be of a guaranteed unique type. This is done by
/// creating a new ZST with the provided marker type struct name.
///
/// The name of the new marker type struct is something we hope to eventually be
/// able to generate automatically from the given newtype name. If there is a way,
/// we don't know how >~<
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use wiwi::nominal::{ Nominal, nominal };
/// // type NewType = Nominal<String, NewTypeMarker>;
/// nominal!(NewType, marker: NewTypeMarker, wraps: String);
///
/// // these two are identical
/// let item: NewType = NewType::new(String::new());
/// let item: Nominal<String, NewTypeMarker> = Nominal::new(String::new());
///
/// // and of course, it's a type alias
/// let item: NewType = Nominal::new(String::new());
/// let item: Nominal<String, NewTypeMarker> = NewType::new(String::new());
/// ```
///
/// The macro does indeed create a unique newtype:
///
/// ```compile_fail
/// # // TODO: use err code E0308 (currently nightly only)
/// # use wiwi::nominal::nominal;
/// # nominal!(NewType, marker: NewTypeMarker, wraps: String);
/// nominal!(AnotherNewType, marker: AnotherNewTypeMarker, wraps: String);
///
/// let item: NewType = NewType::new(String::new());
/// // this won't compile
/// let another_item: NewType = AnotherNewType::new(String::new());
/// ```
///
/// Controlling visibility of the type alias / marker struct:
///
/// ```
/// mod inner {
///    # use wiwi::nominal::nominal;
///    nominal!(pub NewType, marker: NewTypeMarker, wraps: String);
///    //   ↑
/// }
///
/// let item = inner::NewType::new(String::new());
/// ```
///
/// Private visibility is default (like the rest of Rust visibilities):
///
/// ```compile_fail
/// # // TODO: use err code E0603 (currently nightly only)
/// mod inner {
///    # use wiwi::nominal::nominal;
///    nominal!(NewType, marker: NewTypeMarker, wraps: String);
///    //       ↑ no `pub`
/// }
///
/// // this won't compile
/// let item = inner::NewType::new(String::new());
/// ```
///
/// Other visibilities work too, of course:
///
/// ```compile_fail
/// # // TODO: use err code E0603 (currently nightly only)
/// mod outer {
///    mod inner {
///       # // pag
///       # use wiwi::nominal::nominal;
///       nominal!(pub(super) NewType, marker: NewTypeMarker, wraps: String);
///    }
///
///    # fn _maybe_this_fn_decl_shouldnt_be_hidden_i_dont_know() {
///    // this is fine...
///    let item = inner::NewType::new(String::new());
///    # }
/// }
///
/// // but this won't compile
/// let item = outer::inner::NewType::new(String::new());
/// ```
#[macro_export]
macro_rules! nominal {
	($vis:vis $name:ident, marker: $marker:ident, wraps: $( ref <$($lifetimes:lifetime),+> )? $ty:ty) => {
		$vis struct $marker;
		$vis type $name$(<$($lifetimes),+>)? = $crate::nominal::Nominal<$ty, $marker>;
	};
}
pub use nominal;

/// Declare many new nominal types (aliases), in a module
///
/// Usage is more or less identical to [`nominal`], but you define a module
/// inside the macro invocation. Because this macro creates a new module
/// (with the name you specify), and the created module is only used for defining
/// these
/// nominal types, there can be nothing else in there, which we take advantage of
/// to create a `marker` submodule to define marker types in. This way it can have
/// a new namespace just for the marker types, so reusing the newtype name won't
/// collide with anything else.
///
/// So, all of that is to say this macro also saves you a bit of boilerplate
/// declaring names for the newtype ZSTs.
///
/// # Examples
///
/// ```
/// # use wiwi::nominal::nominal_mod;
/// nominal_mod! {
///    pub mod nominal {
///       nominal!(pub NewType, wraps: String);
///    }
/// }
///
/// let item = nominal::NewType::new(String::new());
/// ```
///
/// Still creating newtypes as expected:
///
/// ```compile_fail
/// # // TODO: use err code E0308 (currently nightly only)
/// # use wiwi::nominal::nominal_mod;
/// nominal_mod! {
///    pub mod nominal {
///       nominal!(pub NewType, wraps: String);
///       nominal!(pub AnotherNewType, wraps: String);
///    }
/// }
///
/// let item: nominal::NewType = nominal::NewType::new(String::new());
/// // this won't compile
/// let another_item: nominal::NewType = nominal::AnotherNewType::new(String::new());
/// ```
///
/// Still "just" a type alias:
///
/// ```
/// # use wiwi::nominal::{ Nominal, nominal_mod };
/// # nominal_mod! {
/// #    pub mod nominal {
/// #       nominal!(pub NewType, wraps: String);
/// #    }
/// # }
///
/// let item: nominal::NewType = Nominal::new(String::new());
/// ```
///
/// Created marker structs are in a `marker` submodule:
///
/// ```
/// # use wiwi::nominal::{ Nominal, nominal_mod };
/// # nominal_mod! {
/// #    pub mod nominal {
/// #       nominal!(pub NewType, wraps: String);
/// #       nominal!(pub AnotherNewType, wraps: String);
/// #    }
/// # }
///
/// let item: Nominal<String, nominal::marker::NewType> = nominal::NewType::new(String::new());
/// let item: Nominal<String, nominal::marker::AnotherNewType> = nominal::AnotherNewType::new(String::new());
/// ```
#[macro_export]
macro_rules! nominal_mod {
	{
		$(
			$mod_vis:vis mod $mod_name:ident {
				$( nominal!($item_vis:vis $name:ident, wraps: $( ref <$($lifetimes:lifetime),+> )? $type:ty); )*
			}
		)*
	} => {
		$(
			$mod_vis mod $mod_name {
				pub mod marker {
					$( pub struct $name; )*
				}

				use super::*;
				$( $item_vis type $name$( <$($lifetimes),+> )? = $crate::nominal::Nominal<$type, marker::$name>; )*
			}
		)*
	}
}
pub use nominal_mod;

/// Nominal wrapper struct
///
/// This struct is zero cost; it is simply a type safe wrapper.
///
/// Newtypes are primarily created with assistance from the [`nominal`] and
/// [`nominal_mod`] macros. The macros will help save you the boilerplate of
/// writing the types and declaring unit structs to use as the marker.
#[repr(transparent)]
pub struct Nominal<T, M> {
	item: T,
	marker: PhantomData<M>
}

impl<T, M> Nominal<T, M> {
	/// Creates a nominal struct with the given value
	#[inline]
	pub fn new(item: T) -> Self {
		Self { item, marker: PhantomData }
	}

	/// Unwraps the nominal struct and returns the value
	#[inline]
	pub fn into_inner(self) -> T {
		self.item
	}

	/// Gets a reference to the wrapped value
	///
	/// Note: [`Deref`] is not implemented on purpose, to prevent
	/// unintentional auto-derefs
	// TODO: should we reconsider the above?
	#[inline]
	pub fn as_value_ref(&self) -> &T {
		&self.item
	}

	/// Gets a mut reference to the wrapped value
	///
	/// Note: [`DerefMut`] is not implemented on purpose, to prevent
	/// unintentional auto-derefs
	// TODO: should we reconsider the above?
	#[inline]
	pub fn as_value_mut(&mut self) -> &mut T {
		&mut self.item
	}

	/// Unwraps and rewraps the value as another nominal type, without modifying
	/// the wrapped value
	///
	/// If you're using this function, make sure you know why you're using it!
	/// after all, the whole point of this is to seperate otherwise identical
	/// types into newtypes based on semantic meaning.
	#[inline]
	pub fn with_other_marker<M2>(self) -> Nominal<T, M2> {
		Nominal::new(self.into_inner())
	}

	/// Consumes and maps the wrapped value into another value, wrapping it in
	/// a nominal type with the same marker
	#[inline]
	pub fn map<T2, F>(self, f: F) -> Nominal<T2, M>
	where
		F: FnOnce(T) -> T2
	{
		Nominal::new(f(self.into_inner()))
	}

	/// Consumes and asyncronously maps the wrapped value into another value,
	/// wrapping it in a nominal type with the same marker
	#[inline]
	pub async fn map_async<T2, F, Fu>(self, f: F) -> Nominal<T2, M>
	where
		F: FnOnce(T) -> Fu,
		Fu: IntoFuture<Output = T2>
	{
		Nominal::new(f(self.into_inner()).await)
	}
}

impl<T, M, E> Nominal<Result<T, E>, M> {
	/// Transpose a nominal wrapped [`Result`] into a [`Result`] of a nominal
	/// wrapped value
	///
	/// The value gets wrapped, but the error does not. Both are not otherwise
	/// modified in any way.
	#[inline]
	pub fn transpose(self) -> Result<Nominal<T, M>, E> {
		self.into_inner().map(Nominal::new)
	}

	/// Maps the [`Ok`] value of a [`Result`], wrapping the resulting [`Result`]
	/// in a nominal type with the same marker
	#[inline]
	pub fn map_result_ok<T2, F>(self, f: F) -> Nominal<Result<T2, E>, M>
	where
		F: FnOnce(T) -> T2
	{
		Nominal::new(self.into_inner().map(f))
	}

	/// Maps the [`Err`] value of a [`Result`], wrapping the resulting [`Result`]
	/// in a nominal type with the same marker
	#[inline]
	pub fn map_result_err<E2, F>(self, f: F) -> Nominal<Result<T, E2>, M>
	where
		F: FnOnce(E) -> E2
	{
		Nominal::new(self.into_inner().map_err(f))
	}
}

impl<T, M> Nominal<Option<T>, M> {
	/// Transpose a nominal wrapped [`Option`] into an [`Option`] of a nominal
	/// wrapped value
	///
	/// The value is not otherwise modified in any way.
	#[inline]
	pub fn transpose(self) -> Option<Nominal<T, M>> {
		self.into_inner().map(Nominal::new)
	}

	/// Maps the [`Some`] value of an [`Option`], wrapping the resulting [`Option`]
	/// in a nominal type with the same marker
	#[inline]
	pub fn map_option_some<T2, F>(self, f: F) -> Nominal<Option<T2>, M>
	where
		F: FnOnce(T) -> T2
	{
		Nominal::new(self.into_inner().map(f))
	}
}

impl<T, M> From<T> for Nominal<T, M> {
	#[inline]
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

// delegate trait impls by just calling T's impl

impl<T, M> Clone for Nominal<T, M>
where
	T: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		Self::new(self.as_value_ref().clone())
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		self.as_value_mut().clone_from(source.as_value_ref())
	}
}

impl<T, M> Copy for Nominal<T, M>
where
	T: Copy
{}

impl<T, M> Debug for Nominal<T, M>
where
	T: Debug
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Nominal")
			.field("value", self.as_value_ref())
			.finish()
	}
}

impl<T, M> Display for Nominal<T, M>
where
	T: Display
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_value_ref().fmt(f)
	}
}

impl<T, M> Default for Nominal<T, M>
where
	T: Default
{
	#[inline]
	fn default() -> Self {
		Self::new(T::default())
	}
}

impl<T, M> Hash for Nominal<T, M>
where
	T: Hash
{
	#[inline]
	fn hash<H>(&self, state: &mut H)
	where
		H: Hasher
	{
		self.as_value_ref().hash(state)
	}

	#[inline]
	fn hash_slice<H>(data: &[Self], state: &mut H)
	where
		Self: Sized,
		H: Hasher
	{
		#[expect(clippy::as_conversions, reason = "ptr cast")]
		let slice_ptr = data as *const [Self] as *const [T];

		// SAFETY:
		// - we're repr(transparent)
		// - reborrowing `data` as slice of `T`
		let slice = unsafe { &*slice_ptr };

		T::hash_slice(slice, state)
	}
}

impl<T, M, TR, MR> PartialEq<Nominal<TR, MR>> for Nominal<T, M>
where
	T: PartialEq<TR>
{
	#[inline]
	fn eq(&self, other: &Nominal<TR, MR>) -> bool {
		self.as_value_ref().eq(other.as_value_ref())
	}

	#[expect(
		clippy::partialeq_ne_impl,
		reason = "T might have overridden `ne`. we should use it if so"
	)]
	#[inline]
	fn ne(&self, other: &Nominal<TR, MR>) -> bool {
		self.as_value_ref().ne(other.as_value_ref())
	}
}

impl<T, M> Eq for Nominal<T, M>
where
	T: Eq
{}

impl<T, M, TR, MR> PartialOrd<Nominal<TR, MR>> for Nominal<T, M>
where
	T: PartialOrd<TR>
{
	#[inline]
	fn partial_cmp(&self, other: &Nominal<TR, MR>) -> Option<cmp::Ordering> {
		self.as_value_ref().partial_cmp(other.as_value_ref())
	}

	#[inline]
	fn lt(&self, other: &Nominal<TR, MR>) -> bool {
		self.as_value_ref().lt(other.as_value_ref())
	}

	#[inline]
	fn le(&self, other: &Nominal<TR, MR>) -> bool {
		self.as_value_ref().le(other.as_value_ref())
	}

	#[inline]
	fn gt(&self, other: &Nominal<TR, MR>) -> bool {
		self.as_value_ref().gt(other.as_value_ref())
	}

	#[inline]
	fn ge(&self, other: &Nominal<TR, MR>) -> bool {
		self.as_value_ref().ge(other.as_value_ref())
	}
}

impl<T, M> Ord for Nominal<T, M>
where
	T: Ord
{
	#[inline]
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.as_value_ref().cmp(other.as_value_ref())
	}

	#[inline]
	fn max(self, other: Self) -> Self {
		Self::new(self.into_inner().max(other.into_inner()))
	}

	#[inline]
	fn min(self, other: Self) -> Self {
		Self::new(self.into_inner().min(other.into_inner()))
	}

	#[inline]
	fn clamp(self, min: Self, max: Self) -> Self {
		Self::new(self.into_inner().clamp(min.into_inner(), max.into_inner()))
	}
}
