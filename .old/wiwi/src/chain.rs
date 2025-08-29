use crate::prelude::*;

pub use self::array::ArrayChain;
pub use self::generic::{ GenericChain, GenericChainConversion };
pub use self::hashbrown_hashmap::HashMapChain;
pub use self::vec::{ vec_chain, VecChain };

mod array;
mod generic;
mod hashbrown_hashmap;
mod vec;

pub trait Chain
where
	Self: Sized + private::Sealed + Into<Self::Inner> + AsRef<Self::Inner> + AsMut<Self::Inner>,
	Self::Inner: ChainInner<Chain = Self>
{
	type Inner;

	#[inline]
	fn into_inner(self) -> Self::Inner {
		self.into()
	}

	#[inline]
	fn from_inner(inner: Self::Inner) -> Self {
		inner.into()
	}

	#[inline]
	fn as_inner(&self) -> &Self::Inner {
		self.as_ref()
	}

	#[inline]
	fn as_inner_mut(&mut self) -> &mut Self::Inner {
		self.as_mut()
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
	/// ```
	/// # use wiwi::chain::{ Chain as _, VecChain };
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
	fn with_inner<F, Void>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut Self::Inner) -> Void
	{
		let _void = f(self.as_inner_mut());
		self
	}
}

pub trait ChainInner
where
	Self: Sized + private::Sealed + Into<Self::Chain>,
	Self::Chain: Chain<Inner = Self>
{
	type Chain;

	#[inline]
	fn into_chain(self) -> Self::Chain {
		self.into()
	}

	#[inline]
	fn from_chain(chain: Self::Chain) -> Self {
		chain.into()
	}
}

/// Trait implemented on chains and their inner types, allowing you to get a reference
/// to the inner type regardless of if the chain or the inner type is passed in
pub trait AsChainInner<I>
where
	Self: Sized + private::Sealed
{
	fn as_inner(&self) -> &I;
	fn as_inner_mut(&mut self) -> &mut I;
}

/// Trait for output locations that can be passed to a chainer
///
/// # Safety
///
/// Consumers of this trait must call [`store`] before they return again,
/// implementors must make sure that `self` is written to when called, so users
/// can rely on the fact that the output location was written to. For example,
/// users can pass a reference to [`MaybeUninit`] and rely on the fact that it
/// got initialised, and safely call [`assume_init`](MaybeUninit::assume_init).
///
/// [`store`]: OutputStorage::store
pub unsafe trait OutputStorage<T>
where
	Self: Sized + private::OutputStorageSealed<T>
{
	/// # Safety
	///
	/// This can and should only be called once, and you must call it before returning,
	/// so users can rely on the fact that something got stored in `self`
	unsafe fn store(self, item: T);
}

impl<T> private::OutputStorageSealed<T> for &mut T {}
// SAFETY: we always write once to `self`
unsafe impl<T> OutputStorage<T> for &mut T {
	#[inline]
	unsafe fn store(self, item: T) {
		*self = item;
	}
}

impl<T> private::OutputStorageSealed<T> for &mut MaybeUninit<T> {}
// SAFETY: we always write once to `self`
unsafe impl<T> OutputStorage<T> for &mut MaybeUninit<T> {
	#[inline]
	unsafe fn store(self, item: T) {
		self.write(item);
	}
}

impl<T> private::OutputStorageSealed<T> for &mut Option<T> {}
// SAFETY: we always write once to `self`
unsafe impl<T> OutputStorage<T> for &mut Option<T> {
	#[inline]
	unsafe fn store(self, item: T) {
		*self = Some(item);
	}
}

macro_rules! decl_chain {
	{
		$(#[$meta:meta])*
		$(
			generics_decl: [$($generics_decl:tt)*]
			generics_decl_struct_def: [$($generics_decl_struct_def:tt)*]
			generics: [$($generics:tt)*]
		)?
		chain: $chain:ident
		inner: $($inner:tt)+
	} => {
		// the struct declaration
		$(#[$meta])*
		#[must_use = "a chain always takes ownership of itself, performs the operation, then returns itself again"]
		#[repr(transparent)]
		pub struct $chain$(<$($generics_decl_struct_def)*>)? {
			_inner: $($inner)+
		}

		// the private::Sealed impls
		impl$(<$($generics_decl)*>)? $crate::chain::private::Sealed for $chain$(<$($generics)*>)? {}
		impl$(<$($generics_decl)*>)? $crate::chain::private::Sealed for $($inner)+ {}

		// impl Chain
		impl$(<$($generics_decl)*>)? $crate::chain::Chain for $chain$(<$($generics)*>)? {
			type Inner = $($inner)+;
		}

		// impl ChainInner
		impl$(<$($generics_decl)*>)? $crate::chain::ChainInner for $($inner)+ {
			type Chain = $chain$(<$($generics)*>)?;
		}

		impl$(<$($generics_decl)*>)? $crate::chain::AsChainInner<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn as_inner(&self) -> &$($inner)+ {
				&self._inner
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut $($inner)+ {
				&mut self._inner
			}
		}

		impl$(<$($generics_decl)*>)? $crate::chain::AsChainInner<$($inner)+> for $($inner)+ {
			#[inline]
			fn as_inner(&self) -> &$($inner)+ {
				self
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut $($inner)+ {
				self
			}
		}

		// impl From<Inner> for Chain
		impl$(<$($generics_decl)*>)? $crate::prelude::From<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn from(inner: $($inner)+) -> Self {
				Self { _inner: inner }
			}
		}

		// impl From<&Inner> for Chain where Inner: Clone
		impl$(<$($generics_decl)*>)? $crate::prelude::From<&$($inner)+> for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Clone
		{
			#[inline]
			fn from(inner: &$($inner)+) -> Self {
				Self { _inner: inner.clone() }
			}
		}

		// impl From<&mut Inner> for Chain where Inner: Clone
		impl$(<$($generics_decl)*>)? $crate::prelude::From<&mut $($inner)+> for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Clone
		{
			#[inline]
			fn from(inner: &mut $($inner)+) -> Self {
				Self { _inner: inner.clone() }
			}
		}

		// impl From<Chain> for Inner
		impl$(<$($generics_decl)*>)? $crate::prelude::From<$chain$(<$($generics)*>)?> for $($inner)+ {
			#[inline]
			fn from(chain: $chain$(<$($generics)*>)?) -> Self {
				chain._inner
			}
		}

		// impl From<&Chain> for Inner where Inner: Clone
		impl$(<$($generics_decl)*>)? $crate::prelude::From<&$chain$(<$($generics)*>)?> for $($inner)+
		where
			$($inner)+: $crate::prelude::Clone
		{
			#[inline]
			fn from(chain: &$chain$(<$($generics)*>)?) -> Self {
				chain._inner.clone()
			}
		}

		// impl From<&mut Chain> for Inner where Inner: Clone
		impl$(<$($generics_decl)*>)? $crate::prelude::From<&mut $chain$(<$($generics)*>)?> for $($inner)+
		where
			$($inner)+: $crate::prelude::Clone
		{
			#[inline]
			fn from(chain: &mut $chain$(<$($generics)*>)?) -> Self {
				chain._inner.clone()
			}
		}

		// impl AsRef<Inner>
		impl$(<$($generics_decl)*>)? $crate::prelude::AsRef<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn as_ref(&self) -> &$($inner)+ {
				&self._inner
			}
		}

		// impl AsMut<Inner>
		impl$(<$($generics_decl)*>)? $crate::prelude::AsMut<$($inner)+> for $chain$(<$($generics)*>)? {
			#[inline]
			fn as_mut(&mut self) -> &mut $($inner)+ {
				&mut self._inner
			}
		}

		// impl Clone
		impl$(<$($generics_decl)*>)? $crate::prelude::Clone for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Clone
		{
			#[inline]
			fn clone(&self) -> Self {
				let inner = <Self as $crate::chain::Chain>::as_inner(self);
				let inner = <<Self as $crate::chain::Chain>::Inner as $crate::prelude::Clone>::clone(inner);
				<Self as $crate::chain::Chain>::from_inner(inner)
			}

			#[inline]
			fn clone_from(&mut self, source: &Self) {
				let inner_self = <Self as $crate::chain::Chain>::as_inner_mut(self);
				let inner_source = <Self as $crate::chain::Chain>::as_inner(source);
				<<Self as $crate::chain::Chain>::Inner as $crate::prelude::Clone>::clone_from(inner_self, inner_source)
			}
		}

		// impl Copy
		impl$(<$($generics_decl)*>)? $crate::prelude::Copy for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Copy
		{}

		// impl Debug
		impl$(<$($generics_decl)*>)? $crate::prelude::Debug for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Debug
		{
			#[inline]
			fn fmt(&self, f: &mut $crate::prelude::fmt::Formatter<'_>) -> $crate::prelude::fmt::Result {
				let mut dbg_struct = $crate::prelude::fmt::Formatter::debug_struct(f, stringify!($chain));
				$crate::prelude::fmt::DebugStruct::field(
					&mut dbg_struct,
					"_",
					<Self as $crate::chain::Chain>::as_inner(self)
				);
				$crate::prelude::fmt::DebugStruct::finish(&mut dbg_struct)
			}
		}

		// impl Default
		impl$(<$($generics_decl)*>)? $crate::prelude::Default for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Default
		{
			#[inline]
			fn default() -> Self {
				let inner = <<Self as $crate::chain::Chain>::Inner as Default>::default();
				<Self as $crate::chain::Chain>::from_inner(inner)
			}
		}

		// impl Display
		impl$(<$($generics_decl)*>)? $crate::prelude::Display for $chain$(<$($generics)*>)?
		where
			$($inner)+: $crate::prelude::Display
		{
			#[inline]
			fn fmt(&self, f: &mut $crate::prelude::fmt::Formatter<'_>) -> $crate::prelude::fmt::Result {
				let inner = <Self as $crate::chain::Chain>::as_inner(self);
				<<Self as $crate::chain::Chain>::Inner as Display>::fmt(inner, f)
			}
		}

		// impl Eq
		// impl Ord

		// impl PartialEq/PartialOrd chain <-> inner
		$crate::chain::decl_chain! {
			@impl_partial_cmp
			[$([$($generics_decl)*])?]
			[$($inner)+]

			[$chain$(<$($generics)*>)?]
			[<$chain$(<$($generics)*>)? as $crate::chain::Chain>::as_inner]

			[$($inner)+]
			[$crate::prelude::identity]
		}

		// impl PartialEq/PartialOrd inner <-> chain
		$crate::chain::decl_chain! {
			@impl_partial_cmp
			[$([$($generics_decl)*])?]
			[$($inner)+]

			[$($inner)+]
			[$crate::prelude::identity]

			[$chain$(<$($generics)*>)?]
			[<$chain$(<$($generics)*>)? as $crate::chain::Chain>::as_inner]
		}

		// impl PartialEq/PartialOrd chain <-> chain
		$crate::chain::decl_chain! {
			@impl_partial_cmp
			[$([$($generics_decl)*])?]
			[$($inner)+]

			[$chain$(<$($generics)*>)?]
			[<$chain$(<$($generics)*>)? as $crate::chain::Chain>::as_inner]

			[$chain$(<$($generics)*>)?]
			[<$chain$(<$($generics)*>)? as $crate::chain::Chain>::as_inner]
		}
	};

	{
		@impl_partial_cmp
		[$([$($generics_decl:tt)*])?]
		[$($inner:tt)+]

		[$($left_ty:tt)+]
		[$left_expr:expr]

		[$($right_ty:tt)+]
		[$right_expr:expr]
	} => {
		impl$(<$($generics_decl)*>)? $crate::prelude::PartialEq<$($right_ty)+> for $($left_ty)+
		where
			$($inner)+: $crate::prelude::PartialEq<$($inner)+>
		{
			#[inline]
			fn eq(&self, other: &$($right_ty)+) -> $crate::prelude::std::primitive::bool {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialEq::eq($left_expr(self), $right_expr(other))
				}
			}

			#[expect(
				clippy::partialeq_ne_impl,
				reason = "inner might have overridden ne for whatever reason, and we should use it if so"
			)]
			#[inline]
			fn ne(&self, other: &$($right_ty)+) -> $crate::prelude::std::primitive::bool {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialEq::ne($left_expr(self), $right_expr(other))
				}
			}
		}

		impl$(<$($generics_decl)*>)? $crate::prelude::PartialOrd<$($right_ty)+> for $($left_ty)+
		where
			$($inner)+: $crate::prelude::PartialOrd<$($inner)+>
		{
			#[inline]
			fn partial_cmp(&self, other: &$($right_ty)+) -> $crate::prelude::Option<$crate::prelude::cmp::Ordering> {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialOrd::partial_cmp($left_expr(self), $right_expr(other))
				}
			}

			#[inline]
			fn lt(&self, other: &$($right_ty)+) -> $crate::prelude::std::primitive::bool {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialOrd::lt($left_expr(self), $right_expr(other))
				}
			}

			#[inline]
			fn le(&self, other: &$($right_ty)+) -> $crate::prelude::std::primitive::bool {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialOrd::le($left_expr(self), $right_expr(other))
				}
			}

			#[inline]
			fn gt(&self, other: &$($right_ty)+) -> $crate::prelude::std::primitive::bool {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialOrd::gt($left_expr(self), $right_expr(other))
				}
			}

			#[inline]
			fn ge(&self, other: &$($right_ty)+) -> $crate::prelude::std::primitive::bool {
				$crate::chain::decl_chain! {
					@impl_partial_cmp_helper
					[$($inner)+]
					PartialOrd::ge($left_expr(self), $right_expr(other))
				}
			}
		}
	};

	{
		@impl_partial_cmp_helper
		[$($inner:tt)+]
		$trait:ident::$trait_fn:ident($($stuff:tt)*)
	} => {
		<$($inner)+ as $crate::prelude::$trait<$($inner)+>>::$trait_fn($($stuff)*)
	};
}
use decl_chain;

macro_rules! chain_fn {
	{
		$(#[$meta:meta])*
		$fn_name:ident
		$([$($generics:tt)*])?
		($inner:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $inner = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			$crate::prelude::identity::<()>($body);
			self
		}
	};

	{
		$(#[$meta:meta])*
		self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$crate::prelude::identity::<()>($body);
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe
		$fn_name:ident
		$([$($generics:tt)*])?
		($inner:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $inner = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			$crate::prelude::identity::<()>($body);
			self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$crate::prelude::identity::<()>($body);
			$self
		}
	};

	{
		$(#[$meta:meta])*
		move
		$fn_name:ident
		$([$($generics:tt)*])?
		($inner:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?($self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let mut $inner = <Self as $crate::chain::Chain>::into_inner(self);
			<Self as $crate::chain::Chain>::from_inner($body)
		}
	};

	{
		$(#[$meta:meta])*
		move self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?($self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe move
		$fn_name:ident
		$([$($generics:tt)*])?
		($inner:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?($self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let mut $inner = <Self as $crate::chain::Chain>::into_inner(self);
			<Self as $crate::chain::Chain>::from_inner($body)
		}
	};

	{
		$(#[$meta:meta])*
		unsafe move self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?($self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			$self
		}
	};

	{
		$(#[$meta:meta])*
		void
		$fn_name:ident
		$([$($generics:tt)*])?
		($inner:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $inner = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			let _ = $body;
			self
		}
	};

	{
		$(#[$meta:meta])*
		void self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let _ = $body
			$self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe void
		$fn_name:ident
		$([$($generics:tt)*])?
		($inner:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let $inner = <Self as $crate::chain::Chain>::as_inner_mut(&mut self);
			let _ = $body;
			self
		}
	};

	{
		$(#[$meta:meta])*
		unsafe void self
		$fn_name:ident
		$([$($generics:tt)*])?
		($self:ident $($args:tt)*)
		$(where { $($where_clause:tt)* })?
		=> $body:expr
	} => {
		$(#[$meta])*
		#[inline]
		pub unsafe fn $fn_name$(<$($generics)*>)?(mut $self $($args)*) -> Self
		$(where $($where_clause)*)?
		{
			let _ = $body
			$self
		}
	};
}
use chain_fn;

/// notouchie
mod private {
	/// notouchie
	pub trait Sealed {}
	/// notouchie
	pub trait OutputStorageSealed<T> {}
}
