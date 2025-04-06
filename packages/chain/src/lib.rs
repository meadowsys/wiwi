use core::fmt::{ self, Debug, Display };
use core::hash::{ Hash, Hasher };

mod array;
mod map;
mod string;
mod vec;

pub type ArrayChain<T, const N: usize> = Chain<[T; N]>;
pub type ArrayMutChain<'h, T, const N: usize> = Chain<&'h mut [T; N]>;

pub type StringChain = Chain<String>;
pub type StringMutChain<'h> = Chain<&'h mut String>;

pub type VecChain<T> = Chain<Vec<T>>;
pub type VecMutChain<'h, T> = Chain<&'h mut Vec<T>>;

#[must_use = "a chain always takes ownership of itself, performs the operation, then returns itself again"]
#[repr(transparent)]
pub struct Chain<T> {
	inner: T
}

impl<T> Chain<T> {
	#[inline]
	pub fn from_inner(inner: T) -> Self {
		Self { inner }
	}

	#[inline]
	pub fn into_inner(self) -> T {
		T::from_chain(self)
	}

	#[inline]
	pub fn as_inner(&self) -> &T {
		&self.inner
	}

	#[inline]
	pub fn as_inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	/// Takes a closure that is called, passing in a reference to the inner value
	///
	/// This is useful for if there is something we have not implemented, or you
	/// otherwise need to borrow the inner struct for something, but are in the
	/// middle of some chain. It lets you do otherwise nonchainable operations
	/// inline with other chaining operations, so no need to break the chain c:
	///
	/// # Examples
	///
	// todo fix and unignore this
	/// ```ignore
	/// # use wiwi_chain::{ Chain as _, VecChain };
	/// let chain = VecChain::<usize>::new();
	///
	/// // let's pretend `push` and `reserve` don't already have chainable versions...
	/// let chain = chain
	///    .with_inner(|v| v.reserve(10))
	///    .with_inner(|v| v.push(1))
	///    .with_inner(|v| v.push(2));
	///
	/// assert!(chain.as_inner().len() == 2);
	/// assert!(chain.as_inner().capacity() >= 10);
	/// ```
	#[inline]
	pub fn with_inner(mut self, f: impl FnOnce(&mut T)) -> Self {
		f(&mut self.inner);
		self
	}
}

impl<T> Chain<&T> {
	#[inline]
	pub fn cloned(&self) -> Chain<T>
	where
		T: Clone
	{
		Chain { inner: self.inner.clone() }
	}

	#[inline]
	pub fn copied(&self) -> Chain<T>
	where
		T: Copy
	{
		Chain { inner: *self.inner }
	}
}

impl<T> Chain<&mut T> {
	#[inline]
	pub fn cloned(&self) -> Chain<T>
	where
		T: Clone
	{
		Chain { inner: self.inner.clone() }
	}

	#[inline]
	pub fn copied(&self) -> Chain<T>
	where
		T: Copy
	{
		Chain { inner: *self.inner }
	}
}

impl<'h, T> Chain<&'h &'h T> {
	#[inline]
	pub fn flatten(self) -> Chain<&'h T> {
		Chain { inner: self.inner }
	}
}

impl<'h, T> Chain<&'h mut &'h mut T> {
	#[inline]
	pub fn flatten(self) -> Chain<&'h mut T> {
		Chain { inner: self.inner }
	}
}

impl<T> Chain<Chain<T>> {
	#[inline]
	pub fn flatten(self) -> Chain<T> {
		Chain { inner: self.inner.inner }
	}

	// ?????
	// #[inline]
	// pub fn as_flattened(&self) -> Chain<&T> {
	// 	Chain { inner: &self.inner.inner }
	// }

	// ?????
	// #[inline]
	// pub fn as_flattened_mut(&mut self) -> Chain<&mut T> {
	// 	Chain { inner: &mut self.inner.inner }
	// }
}

// todo I'm not even sure if we should have these below 2 implementations

// impl<'h, T> Chain<&'h &'h mut T> {
// 	#[inline]
// 	pub fn flatten(self) -> Chain<&'h T> {
// 		Chain { inner: self.inner }
// 	}
// }

// impl<'h, T> Chain<&'h mut &'h T> {
// 	#[inline]
// 	pub fn flatten(self) -> Chain<&'h T> {
// 		Chain { inner: self.inner }
// 	}
// }

impl<T, T2> AsRef<T2> for Chain<T>
where
	T: AsRef<T2>
{
	#[inline]
	fn as_ref(&self) -> &T2 {
		self.inner.as_ref()
	}
}

impl<T, T2> AsMut<T2> for Chain<T>
where
	T: AsMut<T2>
{
	#[inline]
	fn as_mut(&mut self) -> &mut T2 {
		self.inner.as_mut()
	}
}

impl<T> Clone for Chain<T>
where
	T: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		self.as_inner().clone().into_chain()
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		self.as_inner_mut().clone_from(source.as_inner())
	}
}

impl<T> Copy for Chain<T>
where
	T: Copy
{}

impl<T> Debug for Chain<T>
where
	T: Debug
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Chain<T>")
			.field("_", self.as_inner())
			.finish()
	}
}

impl<T> Default for Chain<T>
where
	T: Default
{
	#[inline]
	fn default() -> Self {
		T::default().into_chain()
	}
}

impl<T> Display for Chain<T>
where
	T: Display
{
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		T::fmt(self.as_inner(), f)
	}
}

// todo eq

impl<T> Hash for Chain<T>
where
	T: Hash
{
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.as_inner().hash(state)
	}

	#[inline]
	fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) {
		#[expect(clippy::as_conversions, reason = "ptr cast")]
		let ptr = &raw const *data as *const [T];

		// SAFETY: we are repr(transparent),
		// cast ptr is safe to deref
		let data = unsafe { &*ptr };

		T::hash_slice(data, state)
	}
}

// todo Hasher...?

// todo ord

impl<T, T2> PartialEq<T2> for Chain<T>
where
	T: PartialEq<T2>
{
	#[inline]
	fn eq(&self, other: &T2) -> bool {
		self.as_inner().eq(other)
	}

	#[expect(
		clippy::partialeq_ne_impl,
		reason = "inner might have overridden ne for whatever reason, and we should use it if so"
	)]
	#[inline]
	fn ne(&self, other: &T2) -> bool {
		self.as_inner().ne(other)
	}
}

// todo partialord

pub trait ChainInner: Sized {
	#[inline]
	fn from_chain(chain: Chain<Self>) -> Self {
		chain.inner
	}

	#[inline]
	fn into_chain(self) -> Chain<Self> {
		Chain::from_inner(self)
	}

	#[inline]
	fn chain_mut(&mut self) -> Chain<&mut Self> {
		Chain::from_inner(self)
	}
}

impl<T> ChainInner for T {}

/// Trait implemented on chains and their inner types, allowing you to get a reference
/// to the inner type regardless of if the chain or the inner type is passed in
pub trait ChainConversions
where
	Self: Sized + self::sealed::chain_conversions::Sealed
{
	type Inner: Sized;
	type MutChain<'mut_chain>: Sized
	where
		Self: 'mut_chain;

	fn as_inner(&self) -> &Self::Inner;
	fn as_inner_mut(&mut self) -> &mut Self::Inner;
	fn as_mut_chain(&mut self) -> Self::MutChain<'_>;
}

impl<T> ChainConversions for &mut T
where
	T: ChainConversions
{
	type Inner = T::Inner;
	type MutChain<'mut_chain> = T::MutChain<'mut_chain>
	where
		Self: 'mut_chain;

	#[inline]
	fn as_inner(&self) -> &Self::Inner {
		(**self).as_inner()
	}

	#[inline]
	fn as_inner_mut(&mut self) -> &mut Self::Inner {
		(**self).as_inner_mut()
	}

	#[inline]
	fn as_mut_chain(&mut self) -> Self::MutChain<'_> {
		(**self).as_mut_chain()
	}
}

impl<T> self::sealed::chain_conversions::Sealed for &mut T
where
	T: self::sealed::chain_conversions::Sealed
{}

pub trait ChainConversionsOwned: ChainConversions {
	type OwnedChain: Sized;

	fn into_chain(self) -> Self::OwnedChain;
	fn into_inner(self) -> Self::Inner;
}

pub trait WithSelf: Sized {
	/// Takes ownership of the value, passing a mutable reference of it to a
	/// closure, then returning ownership of the value again
	#[inline]
	fn with_self(mut self, f: impl FnOnce(&mut Self)) -> Self {
		f(&mut self);
		self
	}
}

impl<T> WithSelf for T {}

/// # Safety
///
/// By using this trait, implementors of functions are promising to call
/// [`write`](Output::write), so that when the function returns, there is a
/// value written to the output. For example, users can pass a reference to
/// [`MaybeUninit`](core::mem::MaybeUninit) and rely on the fact that it got
/// initialised to safely call [`assume_init`](core::mem::MaybeUninit::assume_init).
///
/// idk how to enforce the above properly using unsafe etc.
pub unsafe trait Output<T>: Sized + self::sealed::output::Sealed<T> {
	/// Stores a value
	fn write(self, item: T);
}

// SAFETY: we write once to `self`
unsafe impl<T> Output<T> for &mut T {
	#[expect(
		clippy::inline_always,
		reason = "same as MaybeUninit::write"
	)]
	#[inline(always)]
	fn write(self, item: T) {
		*self = item;
	}
}
impl<T> self::sealed::output::Sealed<T> for &mut T {}

// SAFETY: we write once to `self`
unsafe impl<T> Output<T> for &mut Option<T> {
	#[expect(
		clippy::inline_always,
		reason = "same as MaybeUninit::write"
	)]
	#[inline(always)]
	fn write(self, item: T) {
		*self = Some(item);
	}
}
impl<T> self::sealed::output::Sealed<T> for &mut Option<T> {}

// SAFETY: we write once to `self`
unsafe impl<T> Output<T> for &mut core::mem::MaybeUninit<T> {
	#[expect(
		clippy::inline_always,
		reason = "same as MaybeUninit::write"
	)]
	#[inline(always)]
	fn write(self, item: T) {
		self.write(item);
	}
}
impl<T> self::sealed::output::Sealed<T> for &mut core::mem::MaybeUninit<T> {}

/// Tool for helping to debug [`Output`] trait usage in debug mode (if `out` is
/// not written to, the function will panic)
///
/// This function should optimise out to a no-op in release mode.
#[inline]
pub fn out_dbg<T, O: Output<T>>(out: O) -> OutputDebug<T, O> {
	OutputDebug {
		inner: out,
		__marker: std::marker::PhantomData
	}
}

#[repr(transparent)]
pub struct OutputDebug<T, O>
where
	O: Output<T>
{
	inner: O,
	__marker: core::marker::PhantomData<fn(T)>
}

impl<T, O> OutputDebug<T, O>
where
	O: Output<T>
{
	/// Unwraps self and returns the inner output (without ever panicking)
	#[inline]
	pub fn into_inner(self) -> O {
		// in cfg(debug_assertions), we have Drop impl,
		// so we need to do a funny to get `inner` out
		#[cfg(debug_assertions)]
		let inner = {
			let this = core::mem::ManuallyDrop::new(self);

			// SAFETY: ManuallyDrop above prevents double drops
			unsafe { core::ptr::read(&raw const this.inner) }
		};

		// in not(cfg(debug_assertions)), we don't have
		// Drop impl, so we can just normally move it out
		#[cfg(not(debug_assertions))]
		let inner = self.inner;

		inner
	}
}

// SAFETY: we write once to `self`
unsafe impl<T, O> Output<T> for OutputDebug<T, O>
where
	O: Output<T>
{
	#[inline]
	fn write(self, item: T) {
		self.into_inner().write(item);
	}
}

impl<T, O> self::sealed::output::Sealed<T> for OutputDebug<T, O>
where
	O: Output<T>
{}

#[cfg(debug_assertions)]
impl<T, O> Drop for OutputDebug<T, O>
where
	O: Output<T>
{
	#[inline]
	fn drop(&mut self) {
		// writing to this slot will prevent this panic from being called (via ManuallyDrop)
		// additionally this drop impl only is enabled if debug assertions is enabled
		panic!("`write` not called on created instance of `Output` (this is probably a bug)")
	}
}

macro_rules! impl_chain_conversions {
	{
		$(#[$meta:meta])*
		[$($generics:tt)*] $inner:ty
	} => {
		$(#[$meta])*
		impl<$($generics)*> $crate::ChainConversions for $crate::Chain<$inner> {
			type Inner = $inner;
			type MutChain<'mut_chain> = $crate::Chain<&'mut_chain mut $inner>
			where
				Self: 'mut_chain;

			#[inline]
			fn as_inner(&self) -> &Self::Inner {
				&self.inner
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				&mut self.inner
			}

			#[inline]
			fn as_mut_chain(&mut self) -> $crate::Chain<&mut $inner> {
				$crate::Chain { inner: &mut self.inner }
			}
		}

		$(#[$meta])*
		impl<'h, $($generics)*> $crate::ChainConversions for $crate::Chain<&'h mut $inner> {
			type Inner = $inner;
			type MutChain<'mut_chain> = $crate::Chain<&'mut_chain mut $inner>
			where
				Self: 'mut_chain;

			#[inline]
			fn as_inner(&self) -> &Self::Inner {
				self.inner
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				self.inner
			}

			#[inline]
			fn as_mut_chain(&mut self) -> $crate::Chain<&mut $inner> {
				$crate::Chain { inner: self.inner }
			}
		}

		$(#[$meta])*
		impl<$($generics)*> $crate::ChainConversions for $inner {
			type Inner = $inner;
			type MutChain<'mut_chain> = $crate::Chain<&'mut_chain mut $inner>
			where
				Self: 'mut_chain;

			#[inline]
			fn as_inner(&self) -> &Self::Inner {
				self
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				self
			}

			#[inline]
			fn as_mut_chain(&mut self) -> $crate::Chain<&mut $inner> {
				$crate::Chain { inner: self }
			}
		}

		$(#[$meta])*
		impl<$($generics)*> $crate::ChainConversionsOwned for $crate::Chain<$inner> {
			type OwnedChain = $crate::Chain<$inner>;

			#[inline]
			fn into_chain(self) -> $crate::Chain<$inner> {
				self
			}

			#[inline]
			fn into_inner(self) -> $inner {
				<$inner as $crate::ChainInner>::from_chain(self)
			}
		}

		$(#[$meta])*
		impl<$($generics)*> $crate::ChainConversionsOwned for $inner {
			type OwnedChain = $crate::Chain<$inner>;

			#[inline]
			fn into_chain(self) -> $crate::Chain<$inner> {
				$crate::Chain::from_inner(self)
			}

			#[inline]
			fn into_inner(self) -> $inner {
				self
			}
		}

		$(#[$meta])*
		impl<$($generics)*> $crate::sealed::chain_conversions::Sealed for $crate::Chain<$inner> {}
		$(#[$meta])*
		impl<'h, $($generics)*> $crate::sealed::chain_conversions::Sealed for $crate::Chain<&'h mut $inner> {}
		$(#[$meta])*
		impl<$($generics)*> $crate::sealed::chain_conversions::Sealed for $inner {}
	};
}
use impl_chain_conversions;

macro_rules! chain_fns {
	{
		@head
		impl [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		impl<$($generics)*> $crate::Chain<$inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}

		$crate::chain_fns! { @head $($stuff)* }
	};

	{
		@head
		impl mut [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		impl<'h, $($generics)*> $crate::Chain<&'h mut $inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}

		$crate::chain_fns! { @head $($stuff)* }
	};

	{
		@head
		impl ref [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		impl<'h, $($generics)*> $crate::Chain<&'h $inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}

		$crate::chain_fns! { @head $($stuff)* }
	};

	{
		@head
		impl and_mut [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		impl<$($generics)*> $crate::Chain<$inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}

		#[warn(missing_docs)]
		impl<'h, $($generics)*> $crate::Chain<&'h mut $inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}

		$crate::chain_fns! { @head $($stuff)* }
	};

	{
		@head
		$(doc $doc:literal $(($doc_link_to:literal))?)?
		$(#[$meta:meta])*
		fn
		$($stuff:tt)*
	} => {};

	{
		@head
		$(doc $doc:literal $(($doc_link_to:literal))?)?
		$(#[$meta:meta])*
		unsafe fn
		$($stuff:tt)*
	} => {};

	{ @head impl $impl_type:ident $($stuff:tt)* } => {
		compile_error!(concat!("unrecognised impl type found: `", stringify!($impl_type), "`"));
	};

	{ @head } => {};

	{
		@impl
		impl [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! { @impl $($stuff)* }
	};

	{
		@impl
		impl owned [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! { @impl $($stuff)* }
	};

	{
		@impl
		impl mut [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! { @impl $($stuff)* }
	};

	{
		@impl
		impl ref [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! { @impl $($stuff)* }
	};

	{
		@impl
		$(doc $doc:literal $(($doc_link_to:literal))?)?
		$(#[$meta:meta])*
		fn $fn_name:ident$([$($generics:tt)*])?
		($inner:ident $($params:tt)*)
		$(where { $($where:tt)* })?
		$(-> $return_type:ty)?
		{ $($impl:tt)* }

		$($stuff:tt)*
	} => {
		#[inline]
		$(#[$meta])*
		$(
			#[doc = ""]
			#[doc = concat!(
				"See documentation for [`",
				$doc,
				"`]",
				$(
					"(",
					$doc_link_to,
					")",
				)?
				" for more details on the underlying function."
			)]
		)?
		pub fn $fn_name$(<$($generics)*>)?(mut self $($params)*)
		-> $crate::chain_fns! { @return_type_helper $($return_type)? }
		$(where $($where)*)?
		{
			$crate::chain_fns! { @rest_helper self $inner $($return_type)? { $($impl)* } }
		}

		$crate::chain_fns! { @impl $($stuff)* }
	};

	{
		@impl
		$(doc $doc:literal $(($doc_link_to:literal))?)?
		$(#[$meta:meta])*
		unsafe fn $fn_name:ident$([$($generics:tt)*])?
		($inner:ident $($params:tt)*)
		$(where { $($where:tt)* })?
		$(-> $return_type:ty)?
		{ $($impl:tt)* }

		$($stuff:tt)*
	} => {
		#[inline]
		$(#[$meta])*
		$(
			#[doc = ""]
			#[doc = "# Safety"]
			#[doc = ""]
			#[doc = concat!(
				"You must uphold safety invariants of [`",
				$doc,
				"`]",
				$(
					"(",
					$doc_link_to,
					")",
				)?
				"."
			)]
			#[doc = ""]
			#[doc = concat!(
				"See documentation for [`",
				$doc,
				"`]",
				$(
					"(",
					$doc_link_to,
					")",
				)?
				" for more details on the underlying function."
			)]
		)?
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($params)*)
		-> $crate::chain_fns! { @return_type_helper $($return_type)? }
		$(where $($where)*)?
		{
			$crate::chain_fns! { @rest_helper self $inner $($return_type)? { $($impl)* } }
		}

		$crate::chain_fns! { @impl $($stuff)* }
	};

	{ @impl } => {};

	{ @return_type_helper } => { Self };
	{ @return_type_helper $type:ty } => { $crate::Chain<$type> };

	{ @rest_helper $self:ident $inner:ident { $($impl:tt)*} } => {
		let $inner = <Self as $crate::ChainConversions>::as_inner_mut(&mut $self);
		let _: () = { $($impl)* };
		$self
	};
	{ @rest_helper $self:ident $inner:ident $type:ty { $($impl:tt)*} } => {
		// shushes the unused_mut warning
		// I could modify the macro more to not emit `mut self` in the parameter
		// if it isn't needed, but... this is simpler in code I have to type, and
		// compiler is surely smart enough to remove this
		let _ = &mut $self;

		let $inner = $self.into_inner();
		let inner = { $($impl)* };
		$crate::Chain::from_inner(inner)
	};

	{ $($stuff:tt)* } => {
		$crate::chain_fns! { @head $($stuff)* }
	};
}
use chain_fns;

#[allow(
	unused_imports,
	reason = "internal prelude"
)]
#[expect(
	clippy::allow_attributes,
	reason = "internal prelude"
)]
mod prelude_internal {
	pub(crate) use crate::{
		Chain,
		ChainConversions,
		Output,
		chain_fns,
		impl_chain_conversions
	};
}

/// notouchie
mod sealed {
	/// notouchie
	pub mod output {
		/// notouchie
		pub trait Sealed<T> {}
	}

	/// notouchie
	pub mod chain_conversions {
		/// notouchie
		pub trait Sealed {}
	}
}
