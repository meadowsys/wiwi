use self::sealed::*;

pub use self::array::{ ArrayChain, ArrayMutChain };
pub use self::string::{ StringChain, StringMutChain };
pub use self::vec::{ VecChain, VecMutChain };

mod array;
mod string;
mod vec;

pub trait Chain: Sized + ChainSealed {
	type Inner: ChainInner<Chain = Self>;

	fn from_inner(inner: Self::Inner) -> Self;

	#[inline]
	fn into_inner(self) -> Self::Inner {
		Self::Inner::from_chain(self)
	}

	/// Takes a closure that is called, passing in a reference to the inner value
	///
	/// This is useful for if there is something we have not implemented, or you
	/// otherwise need to borrow the inner struct for something, but are in the
	/// middle of some chain. It lets you do otherwise nonchainable operations
	/// inline with other chaining operations, so no need to break the chain c:
	///
	/// The closure passed in is allowed to return anything, but the return value
	/// is simply ignored. This makes it so you can call a function for its side
	/// effect, even if that function returns something you wouldn't have needed
	/// anyways.
	///
	/// For example, [`MaybeUninit::write`] returns `&mut T`, but you might not
	/// need that reference, so instead of writing `
	/// .with_inner(|val| { val.write(...); })`, you can simply write
	/// `.with_inner(|val| val.write(...))`.
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
	fn with_inner<F, Void>(self, f: F) -> Self
	where
		F: FnOnce(&mut Self::Inner) -> Void;
}

pub trait ChainInner: Sized + ChainInnerSealed {
	type Chain: Chain<Inner = Self>;

	fn from_chain(chain: Self::Chain) -> Self;

	#[inline]
	fn into_chain(self) -> Self::Chain {
		Self::Chain::from_inner(self)
	}
}

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
	fn with_self<Void>(mut self, f: impl FnOnce(&mut Self) -> Void) -> Self {
		let _ = f(&mut self);
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

macro_rules! decl_chain {
	{
		$(#[$meta:meta])*
		struct $chain:ident;
		inner $inner:ty;
	} => {
		$crate::decl_chain! {
			$(#[$meta])*
			struct $chain[];
			impl[] $chain;
			inner $inner;
		}
	};

	{
		$(#[$meta:meta])*
		struct $chain:ident[$($chain_decl_generics:tt)*];
		impl[$($chain_impl_generics:tt)*] $chain_impl:ty;
		$(where { $($where:tt)* };)?
		inner $inner:ty;
	} => {
		$(#[$meta])*
		#[must_use = "a chain always takes ownership of itself, performs the operation, then returns itself again"]
		#[repr(transparent)]
		pub struct $chain<$($chain_decl_generics)*>
		$(where $($where)*)?
		{
			__inner: $inner
		}

		impl<$($chain_impl_generics)*> $crate::Chain for $chain_impl
		$(where $($where)*)?
		{
			type Inner = $inner;

			#[inline]
			fn from_inner(inner: $inner) -> Self {
				Self { __inner: inner }
			}

			#[inline]
			fn with_inner<F, Void>(mut self, f: F) -> Self
			where
				F: FnOnce(&mut Self::Inner) -> Void
			{
				let _ = f(&mut self.__inner);
				self
			}
		}

		impl<$($chain_impl_generics)*> $crate::ChainInner for $inner
		$(where $($where)*)?
		{
			type Chain = $chain_impl;

			#[inline]
			fn from_chain(chain: $chain_impl) -> Self {
				chain.__inner
			}
		}

		impl<$($chain_impl_generics)*> $crate::ChainSealed for $chain_impl
		$(where $($where)*)?
		{}

		impl<$($chain_impl_generics)*> $crate::ChainInnerSealed for $inner
		$(where $($where)*)?
		{}

		impl<$($chain_impl_generics)*> ::core::clone::Clone for $chain_impl
		where
			$inner: ::core::clone::Clone
		{
			#[inline]
			fn clone(&self) -> Self {
				let clone = <$inner as ::core::clone::Clone>::clone(&self.__inner);
				<Self as $crate::Chain>::from_inner(clone)
			}

			#[inline]
			fn clone_from(&mut self, other: &Self) {
				<$inner as ::core::clone::Clone>::clone_from(
					&mut self.__inner,
					&other.__inner
				);
			}
		}

		impl<$($chain_impl_generics)*> ::core::default::Default for $chain_impl
		where
			$inner: ::core::default::Default
		{
			#[inline]
			fn default() -> Self {
				let default = <$inner as ::core::default::Default>::default();
				<Self as $crate::Chain>::from_inner(default)
			}
		}

		// todo this doesn't compile because `String: Copy` is a "trivial bound"
		// https://github.com/rust-lang/rust/issues/48214
		// smh
		//
		// impl<$($chain_impl_generics)*> ::core::marker::Copy for $chain_impl
		// where
		// 	$inner: ::core::marker::Copy
		// {}

		// todo more standard traits?
	};
}
use decl_chain;

macro_rules! impl_chain_conversions {
	{
		impl chain [$($impl_chain_generics:tt)*] $impl_chain:ty;
		impl chain_mut [$($impl_chain_mut_generics:tt)*] $impl_chain_mut:ty;
		impl inner [$($impl_inner_generics:tt)*] $impl_inner:ty;
		type inner $inner_type:ty;
		type mut_chain $mut_chain_type:ty;
	} => {
		impl<$($impl_chain_generics)*> $crate::ChainConversions for $impl_chain {
			type Inner = $inner_type;
			type MutChain<'mut_chain> = $mut_chain_type
			where
				Self: 'mut_chain;

			#[inline]
			fn as_inner(&self) -> &Self::Inner {
				&self.__inner
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				&mut self.__inner
			}

			#[inline]
			fn as_mut_chain(&mut self) -> Self::MutChain<'_> {
				Self::MutChain { __inner: &mut self.__inner }
			}
		}

		impl<$($impl_chain_mut_generics)*> $crate::ChainConversions for $impl_chain_mut {
			type Inner = $inner_type;
			type MutChain<'mut_chain> = $mut_chain_type
			where
				Self: 'mut_chain;

			#[inline]
			fn as_inner(&self) -> &Self::Inner {
				self.__inner
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				self.__inner
			}

			#[inline]
			fn as_mut_chain(&mut self) -> Self::MutChain<'_> {
				Self::MutChain { __inner: self.__inner }
			}
		}

		impl<$($impl_inner_generics)*> $crate::ChainConversions for $impl_inner {
			type Inner = $inner_type;
			type MutChain<'mut_chain> = $mut_chain_type
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
			fn as_mut_chain(&mut self) -> Self::MutChain<'_> {
				Self::MutChain { __inner: self }
			}
		}

		impl<$($impl_chain_generics)*> $crate::ChainConversionsSealed for $impl_chain {}
		impl<$($impl_chain_mut_generics)*> $crate::ChainConversionsSealed for $impl_chain_mut {}
		impl<$($impl_inner_generics)*> $crate::ChainConversionsSealed for $impl_inner {}
	};
}
use impl_chain_conversions;

macro_rules! chain_fns {
	{
		impl chain [$($impl_chain_generics:tt)*] $impl_chain:ty;
		impl chain_mut [$($impl_chain_mut_generics:tt)*] $impl_chain_mut:ty;

		$($stuff:tt)*
	} => {
		impl<$($impl_chain_generics)*> $impl_chain {
			$crate::chain_fns! { @impl $($stuff)* }
		}

		impl<$($impl_chain_mut_generics)*> $impl_chain_mut {
			$crate::chain_fns! { @impl $($stuff)* }
		}
	};

	{
		@impl
		$(#[$meta:meta])*
		fn $fn_name:ident$([$($generics:tt)*])?
		($inner:ident $($params:tt)*)
		$(where { $($where:tt)* })?
		{ $($impl:tt)* }

		$($stuff:tt)*
	} => {
		#[inline]
		$(#[$meta])*
		pub fn $fn_name$(<$($generics)*>)?(mut self $($params)*) -> Self
		$(where $($where)*)?
		{
			let $inner = <Self as $crate::ChainConversions>::as_inner_mut(&mut self);
			let _: () = { $($impl)* };
			self
		}

		$crate::chain_fns! { @impl $($stuff)* }
	};

	{
		@impl
		$(#[$meta:meta])*
		unsafe fn $fn_name:ident$([$($generics:tt)*])?
		($inner:ident $($params:tt)*)
		$(where { $($where:tt)* })?
		{ $($impl:tt)* }

		$($stuff:tt)*
	} => {
		#[inline]
		$(#[$meta])*
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($params)*) -> Self
		$(where $($where)*)?
		{
			let $inner = <Self as $crate::ChainConversions>::as_inner_mut(&mut self);
			let _: () = { $($impl)* };
			self
		}

		$crate::chain_fns! { @impl $($stuff)* }
	};

	{ @impl } => {};
}
use chain_fns;

/// notouchie
mod sealed {
	/// notouchie
	pub trait ChainSealed {}

	/// notouchie
	pub trait ChainInnerSealed {}

	/// notouchie
	pub trait OutputSealed<T> {}

	/// notouchie
	pub trait ChainConversionsSealed {}
}
