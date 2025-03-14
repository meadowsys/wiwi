use self::sealed::*;

mod array;
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

impl<T> Default for Chain<T>
where
	T: Default
{
	#[inline]
	fn default() -> Self {
		T::default().into_chain()
	}
}

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
	Self: Sized + ChainConversionsSealed
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

impl<T> ChainConversionsSealed for &mut T
where
	T: ChainConversionsSealed
{}

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
pub unsafe trait Output<T>: Sized + OutputSealed<T> {
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
impl<T> OutputSealed<T> for &mut T {}

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
impl<T> OutputSealed<T> for &mut Option<T> {}

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
impl<T> OutputSealed<T> for &mut core::mem::MaybeUninit<T> {}

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

impl<T, O> OutputSealed<T> for OutputDebug<T, O>
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
		[$($generics:tt)*] $inner:ty
	} => {
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

		impl<$($generics)*> $crate::ChainConversionsSealed for $crate::Chain<$inner> {}
		impl<'h, $($generics)*> $crate::ChainConversionsSealed for $crate::Chain<&'h mut $inner> {}
		impl<$($generics)*> $crate::ChainConversionsSealed for $inner {}
	};
}
use impl_chain_conversions;

macro_rules! chain_fns {
	{
		impl [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			impl owned [$($generics)*] $inner;
			$($stuff)*
		}

		$crate::chain_fns! {
			impl mut [$($generics)*] $inner;
			$($stuff)*
		}
	};

	{
		impl owned [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		impl<$($generics)*> $crate::Chain<$inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}
	};

	{
		impl mut [$($generics:tt)*] $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		impl<'h, $($generics)*> $crate::Chain<&'h mut $inner> {
			$crate::chain_fns! { @impl $($stuff)* }
		}
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
	{ @return_type_helper $type:ty } => { $type };

	{ @rest_helper $self:ident $inner:ident { $($impl:tt)*} } => {
		let $inner = <Self as $crate::ChainConversions>::as_inner_mut(&mut $self);
		let _: () = { $($impl)* };
		$self
	};
	{ @rest_helper $self:ident $inner:ident $type:ty { $($impl:tt)*} } => {
		let $inner = $self.into_inner();
		$($impl)*
	};
}
use chain_fns;

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
	pub trait OutputSealed<T> {}

	/// notouchie
	pub trait ChainConversionsSealed {}
}
