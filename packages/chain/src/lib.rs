use core::fmt::{ self, Debug, Display };
use core::hash::{ Hash, Hasher };
#[cfg(feature = "serde")]
use serde::{ Deserialize, Deserializer, Serialize, Serializer };

pub use self::do_if::*;

pub mod do_if;
pub mod types;

mod array;
mod map;
mod set;
mod string;
mod vec;

#[must_use = "a chain always takes ownership of itself, performs the operation, then returns itself again"]
#[repr(transparent)]
pub struct Chain<T> {
	__inner: T
}

impl<T> Chain<T> {
	#[inline]
	pub fn from_inner(inner: T) -> Self {
		Self { __inner: inner }
	}

	#[inline]
	pub fn into_inner(self) -> T {
		T::from_chain(self)
	}

	#[inline]
	pub fn as_inner(&self) -> &T {
		&self.__inner
	}

	#[inline]
	pub fn as_inner_mut(&mut self) -> &mut T {
		&mut self.__inner
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
		f(self.as_inner_mut());
		self
	}

	#[inline]
	pub fn do_if(
		self,
		condition: bool,
		f: impl FnOnce(Self) -> Self
	) -> Self {
		self.do_if_cond::<IsTrue, _>(
			condition,
			|c, _| f(c)
		)
	}

	#[inline]
	pub fn do_if_some<Some>(
		self,
		condition: Option<Some>,
		f: impl FnOnce(Self, Some) -> Self
	) -> Self {
		self.do_if_cond::<IsSome, _>(condition, f)
	}

	#[inline]
	pub fn do_if_ok<Ok, Err>(
		self,
		condition: Result<Ok, Err>,
		f: impl FnOnce(Self, Ok) -> Self
	) -> Self {
		self.do_if_cond::<IsOk, _>(condition, f)
	}

	#[inline]
	pub fn do_if_err<Ok, Err>(
		self,
		condition: Result<Ok, Err>,
		f: impl FnOnce(Self, Err) -> Self
	) -> Self {
		self.do_if_cond::<IsErr, _>(condition, f)
	}

	#[inline]
	pub fn do_if_cond<Condition, Wrapped>(
		self,
		condition: Wrapped,
		f: impl FnOnce(Self, Condition::Unwrapped) -> Self
	) -> Self
	where
		Condition: DoIf<Wrapped>
	{
		Condition::run(self, condition, f)
	}
}

impl<T> Chain<&T> {
	#[inline]
	pub fn cloned(&self) -> Chain<T>
	where
		T: Clone
	{
		(**self.as_inner()).clone().into_chain()
	}

	#[inline]
	pub fn copied(&self) -> Chain<T>
	where
		T: Copy
	{
		(**self.as_inner()).into_chain()
	}
}

impl<T> Chain<&mut T> {
	#[inline]
	pub fn cloned(&self) -> Chain<T>
	where
		T: Clone
	{
		(**self.as_inner()).clone().into_chain()
	}

	#[inline]
	pub fn copied(&self) -> Chain<T>
	where
		T: Copy
	{
		(**self.as_inner()).into_chain()
	}
}

impl<'h, T> Chain<&'h &'h T> {
	#[inline]
	pub fn flatten(self) -> Chain<&'h T> {
		(*self.into_inner()).into_chain()
	}
}

impl<'h, T> Chain<&'h mut &'h mut T> {
	#[inline]
	pub fn flatten(self) -> Chain<&'h mut T> {
		(*self.into_inner()).into_chain()
	}
}

impl<T> Chain<Chain<T>> {
	#[inline]
	pub fn flatten(self) -> Chain<T> {
		self.into_inner()
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
		T::as_ref(self.as_inner())
	}
}

impl<T, T2> AsMut<T2> for Chain<T>
where
	T: AsMut<T2>
{
	#[inline]
	fn as_mut(&mut self) -> &mut T2 {
		T::as_mut(self.as_inner_mut())
	}
}

impl<T> Clone for Chain<T>
where
	T: Clone
{
	#[inline]
	fn clone(&self) -> Self {
		T::clone(self.as_inner()).into_chain()
	}

	#[inline]
	fn clone_from(&mut self, source: &Self) {
		T::clone_from(self.as_inner_mut(), source.as_inner())
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

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for Chain<T>
where
	T: Deserialize<'de>
{
	#[inline]
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		T::deserialize(deserializer)
			.map(|inner| inner.into_chain())
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
		T::hash(self.as_inner(), state)
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

impl<T> IntoIterator for Chain<T>
where
	T: IntoIterator
{
	type Item = T::Item;
	type IntoIter = T::IntoIter;

	#[inline]
	fn into_iter(self) -> T::IntoIter {
		self.into_inner().into_iter()
	}
}

impl<'h, T> IntoIterator for &'h Chain<T>
where
	&'h T: IntoIterator
{
	type Item = <&'h T as IntoIterator>::Item;
	type IntoIter = <&'h T as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> <&'h T as IntoIterator>::IntoIter {
		self.as_inner().into_iter()
	}
}

impl<'h, T> IntoIterator for &'h mut Chain<T>
where
	&'h mut T: IntoIterator
{
	type Item = <&'h mut T as IntoIterator>::Item;
	type IntoIter = <&'h mut T as IntoIterator>::IntoIter;

	#[inline]
	fn into_iter(self) -> <&'h mut T as IntoIterator>::IntoIter {
		self.as_inner_mut().into_iter()
	}
}

// todo ord

impl<T, T2> PartialEq<T2> for Chain<T>
where
	T: PartialEq<T2>
{
	#[inline]
	fn eq(&self, other: &T2) -> bool {
		T::eq(self.as_inner(), other)
	}

	#[expect(
		clippy::partialeq_ne_impl,
		reason = "inner might have overridden ne for whatever reason, and we should use it if so"
	)]
	#[inline]
	fn ne(&self, other: &T2) -> bool {
		T::ne(self.as_inner(), other)
	}
}

#[cfg(feature = "serde")]
impl<T> Serialize for Chain<T>
where
	T: Serialize
{
	#[inline]
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer
	{
		T::serialize(self.as_inner(), serializer)
	}
}

// todo partialord

pub trait ChainInner: Sized {
	#[inline]
	fn from_chain(chain: Chain<Self>) -> Self {
		chain.into_inner()
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
				$crate::Chain::as_inner(self)
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				$crate::Chain::as_inner_mut(self)
			}

			#[inline]
			fn as_mut_chain(&mut self) -> $crate::Chain<&mut $inner> {
				let inner = $crate::Chain::as_inner_mut(self);
				$crate::Chain::from_inner(inner)
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
				&**$crate::Chain::as_inner(self)
			}

			#[inline]
			fn as_inner_mut(&mut self) -> &mut Self::Inner {
				&mut **$crate::Chain::as_inner_mut(self)
			}

			#[inline]
			fn as_mut_chain(&mut self) -> $crate::Chain<&mut $inner> {
				let inner = $crate::Chain::as_inner_mut(self);
				$crate::Chain::from_inner(&mut **inner)
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
				$crate::Chain::from_inner(self)
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
				$crate::Chain::into_inner(self)
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
	{ @head @head @head @head @head $($stuff:tt)* } => {
		compile_error!("you have huge brain use your own macro properly 5head");
	};

	{
		@head
		$(#[$meta:meta])*
		impl [$($generics:tt)*] $({ doc $($doc_type:tt)* })? $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		$(#[$meta])*
		impl<$($generics)*> $crate::Chain<$inner> {
			$crate::chain_fns! {
				@impl
				$({ doctype $($doc_type)* })?
				$($stuff)*
			}
		}

		$crate::chain_fns! {
			@head
			$($stuff)*
		}
	};

	{
		@head
		$(#[$meta:meta])*
		impl mut [$($generics:tt)*] $({ doc $($doc_type:tt)* })? $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		$(#[$meta])*
		impl<'h, $($generics)*> $crate::Chain<&'h mut $inner> {
			$crate::chain_fns! {
				@impl
				$({ doctype $($doc_type)* })?
				$($stuff)*
			}
		}

		$crate::chain_fns! {
			@head
			$($stuff)*
		}
	};

	{
		@head
		$(#[$meta:meta])*
		impl ref [$($generics:tt)*] $({ doc $($doc_type:tt)* })? $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		$(#[$meta])*
		impl<'h, $($generics)*> $crate::Chain<&'h $inner> {
			$crate::chain_fns! {
				@impl
				$({ doctype $($doc_type)* })?
				$($stuff)*
			}
		}

		$crate::chain_fns! {
			@head
			$($stuff)*
		}
	};

	{
		@head
		$(#[$meta:meta])*
		impl and_mut [$($generics:tt)*] $({ doc $($doc_type:tt)* })? $inner:ty;

		$($stuff:tt)*
	} => {
		#[warn(missing_docs)]
		$(#[$meta])*
		impl<$($generics)*> $crate::Chain<$inner> {
			$crate::chain_fns! {
				@impl
				$({ doctype $($doc_type)* })?
				$($stuff)*
			}
		}

		#[warn(missing_docs)]
		$(#[$meta])*
		impl<'h, $($generics)*> $crate::Chain<&'h mut $inner> {
			$crate::chain_fns! {
				@impl
				$({ doctype $($doc_type)* })?
				$($stuff)*
			}
		}

		$crate::chain_fns! {
			@head
			$($stuff)*
		}
	};

	{
		@head
		$(#[$meta:meta])*
		impl $impl_type:ident $($stuff:tt)*
	} => {
		compile_error!(concat!(
			"unrecognised impl type found: `",
			stringify!($impl_type),
			"`"
		));
	};

	{
		@head
		$({ doctype $($doc_type:tt)* })?

		$(doc [$($doc:tt)+]$(($($doc_link_to:tt)+))?)?
		$(#[$meta:meta])*
		fn
		$($stuff:tt)*
	} => {};

	{
		@head
		$({ doctype $($doc_type:tt)* })?

		$(doc [$($doc:tt)+]$(($($doc_link_to:tt)+))?)?
		$(#[$meta:meta])*
		unsafe fn
		$($stuff:tt)*
	} => {};

	{ @head } => {};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?

		$(#[$meta:meta])*
		impl [$($generics:tt)*] $({ doc $($_doc_type:tt)* })? $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			@impl
			$({ doctype $($doc_type)* })?
			$($stuff)*
		}
	};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?

		$(#[$meta:meta])*
		impl $($impl_type:ident)? [$($generics:tt)*] $({ doc $($_doc_type:tt)* })? $inner:ty;

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			@impl
			$({ doctype $($doc_type)* })?
			$($stuff)*
		}
	};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?

		doc [Self]$(($($doc_link_to:tt)+))?
		$(#[$meta:meta])*
		fn $fn_name:ident

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			@impl
			$({ doctype $($doc_type)* })?

			doc [Self::$fn_name]$(($($doc_link_to)+))?
			$(#[$meta])*
			fn $fn_name

			$($stuff)*
		}
	};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?

		doc [Self]$(($($doc_link_to:tt)+))?
		$(#[$meta:meta])*
		unsafe fn $fn_name:ident

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			@impl
			$({ doctype $($doc_type)* })?

			doc [Self::$fn_name]$(($($doc_link_to)+))?
			$(#[$meta])*
			unsafe fn $fn_name

			$($stuff)*
		}
	};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?

		$(doc [$($doc:tt)+]$(($($doc_link_to:tt)+))?)?
		$(#[$meta:meta])*
		fn $fn_name:ident$([$($generics:tt)*])?
		($inner:ident $($params:tt)*)
		$(where { $($where:tt)* })?
		$(-> $return_type:ty)?
		{ $($impl:tt)* }

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			@helper doc
			{
				#[inline]
				$(#[$meta])*
			}
			$(doc { [$($doc)+]$(($($doc_link_to)+))? })?
			$(doctype { $($doc_type)* })?

			item
			pub fn $fn_name$(<$($generics)*>)?(mut self $($params)*)
			-> $crate::chain_fns! { @helper return_type $($return_type)? }
			$(where $($where)*)?
			{
				$crate::chain_fns! {
					@helper rest
					self
					$inner
					$($return_type)?
					{ $($impl)* }
				}
			}
		}

		$crate::chain_fns! {
			@impl
			$({ doctype $($doc_type)* })?
			$($stuff)*
		}
	};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?

		$(doc [$($doc:tt)+]$(($($doc_link_to:tt)+))?)?
		$(#[$meta:meta])*
		unsafe fn $fn_name:ident$([$($generics:tt)*])?
		($inner:ident $($params:tt)*)
		$(where { $($where:tt)* })?
		$(-> $return_type:ty)?
		{ $($impl:tt)* }

		$($stuff:tt)*
	} => {
		$crate::chain_fns! {
			@helper doc unsafe
			{
				#[inline]
				$(#[$meta])*
			}
			$(doc { [$($doc)+]$(($($doc_link_to)+))? })?
			$(doctype { $($doc_type)* })?

			item
			pub unsafe fn $fn_name$(<$($generics)*>)?(mut self $($params)*)
			-> $crate::chain_fns! { @helper return_type $($return_type)? }
			$(where $($where)*)?
			{
				$crate::chain_fns! {
					@helper rest
					self
					$inner
					$($return_type)?
					{ $($impl)* }
				}
			}
		}

		$crate::chain_fns! {
			@impl
			$({ doctype $($doc_type)* })?
			$($stuff)*
		}
	};

	{
		@impl
		$({ doctype $($doc_type:tt)* })?
	} => {};

	{
		@helper doc $(unsafe)?
		{ $(#[$before_meta:meta])* }
		$(doctype { $doc_type:literal $($doc_type_link_to:literal)? })?
		item $item:item
	} => {
		$(#[$before_meta])*
		$item
	};

	{
		@helper doc
		{ $(#[$before_meta:meta])* }
		doc { [$doc:literal]$(($doc_link_to:literal))? }
		$(doctype { $doc_type:literal $($doc_type_link_to:literal)? })?
		item $item:item
	} => {
		$crate::chain_fns! {
			@helper doc_impl
			{ $(#[$before_meta])* }
			doc { $doc }
			$(doc_link_to { $doc_link_to })?
			item $item
		}
	};

	{
		@helper doc unsafe
		{ $(#[$before_meta:meta])* }
		doc { [$doc:literal]$(($doc_link_to:literal))? }
		$(doctype { $doc_type:literal $($doc_type_link_to:literal)? })?
		item $item:item
	} => {
		$crate::chain_fns! {
			@helper doc_impl unsafe
			{ $(#[$before_meta])* }
			doc { $doc }
			$(doc_link_to { $doc_link_to })?
			item $item
		}
	};

	// below 2 macro arms branches things
	// should be identical to the 2 below
	// except with doc_link_to removed
	// (need to keep doc_type_link_to to make it trivial to invoke)
	{
		@helper doc
		{ $(#[$before_meta:meta])* }
		doc { [Self::$doc:ident] }
		doctype { $doc_type:literal $($doc_type_link_to:literal)? }
		item $item:item
	} => {
		$crate::chain_fns! {
			@helper doc_impl
			{ $(#[$before_meta])* }
			doc { $doc_type, "::", stringify!($doc) }
			$(doc_link_to { $doc_type_link_to, "::", stringify!($doc) })?
			item $item
		}
	};

	{
		@helper doc unsafe
		{ $(#[$before_meta:meta])* }
		doc { [Self::$doc:ident] }
		doctype { $doc_type:literal $($doc_type_link_to:literal)? }
		item $item:item
	} => {
		$crate::chain_fns! {
			@helper doc_impl unsafe
			{ $(#[$before_meta])* }
			doc { $doc_type, "::", stringify!($doc) }
			$(doc_link_to { $doc_type_link_to, "::", stringify!($doc) })?
			item $item
		}
	};

	// probably broken/out of date
	// {
	// 	@helper doc
	// 	{ $(#[$before_meta:meta])* }
	// 	doc { self::$doc:literal $($doc_link_to:literal)? }
	// 	doctype { $doc_type:literal $($doc_type_link_to:literal)? }
	// 	item $item:item
	// } => {
	// 	$crate::chain_fns! {
	// 		@helper doc_impl
	// 		{ $(#[$before_meta])* }
	// 		doc { $doc_type, "::", $doc }
	// 		$(doc_link_to { $doc_link_to })?
	// 		item $item
	// 	}
	// };

	// probably broken/out of date
	// {
	// 	@helper doc unsafe
	// 	{ $(#[$before_meta:meta])* }
	// 	doc { self::$doc:literal $($doc_link_to:literal)? }
	// 	doctype { $doc_type:literal $($doc_type_link_to:literal)? }
	// 	item $item:item
	// } => {
	// 	$crate::chain_fns! {
	// 		@helper doc_impl unsafe
	// 		{ $(#[$before_meta])* }
	// 		doc { $doc_type, "::", $doc }
	// 		$(doc_link_to { $doc_link_to })?
	// 		item $item
	// 	}
	// };

	// probably broken/out of date
	// {
	// 	@helper doc
	// 	{ $(#[$before_meta:meta])* }
	// 	doc { $doc:literal self::$doc_link_to:literal }
	// 	doctype { $doc_type:literal $doc_type_link_to:literal }
	// 	item $item:item
	// } => {
	// 	$crate::chain_fns! {
	// 		@helper doc_impl
	// 		{ $(#[$before_meta])* }
	// 		doc { $doc }
	// 		doc_link_to { $doc_type_link_to, "::", $doc_link_to }
	// 		item $item
	// 	}
	// };

	// probably broken/out of date
	// {
	// 	@helper doc unsafe
	// 	{ $(#[$before_meta:meta])* }
	// 	doc { $doc:literal self::$doc_link_to:literal }
	// 	doctype { $doc_type:literal $doc_type_link_to:literal }
	// 	item $item:item
	// } => {
	// 	$crate::chain_fns! {
	// 		@helper doc_impl unsafe
	// 		{ $(#[$before_meta])* }
	// 		doc { $doc }
	// 		doc_link_to { $doc_type_link_to, "::", $doc_link_to }
	// 		item $item
	// 	}
	// };

	// probably broken/out of date
	// {
	// 	@helper doc
	// 	{ $(#[$before_meta:meta])* }
	// 	doc { self::$doc:literal $(self::$doc_link_to:literal)? }
	// 	doctype { $doc_type:literal $($doc_type_link_to:literal)? }
	// 	item $item:item
	// } => {
	// 	$crate::chain_fns! {
	// 		@helper doc_impl
	// 		{ $(#[$before_meta])* }
	// 		doc { $doc_type, "::", $doc }
	// 		doc_link_to { $doc_type_link_to, "::", $doc_link_to }
	// 		item $item
	// 	}
	// };

	// probably broken/out of date
	// {
	// 	@helper doc unsafe
	// 	{ $(#[$before_meta:meta])* }
	// 	doc { self::$doc:literal $(self::$doc_link_to:literal)? }
	// 	doctype { $doc_type:literal $($doc_type_link_to:literal)? }
	// 	item $item:item
	// } => {
	// 	$crate::chain_fns! {
	// 		@helper doc_impl unsafe
	// 		{ $(#[$before_meta])* }
	// 		doc { $doc_type, "::", $doc }
	// 		doc_link_to { $doc_type_link_to, "::", $doc_link_to }
	// 		item $item
	// 	}
	// };

	{
		@helper doc_impl
		{ $(#[$before_meta:meta])* }
		doc { $($doc:tt)* }
		$(doc_link_to { $($doc_link_to:tt)* })?
		item $item:item
	} => {
		$(#[$before_meta])*
		#[doc = ""]
		#[doc = concat!(
			"See documentation for [`",
			$($doc)*,
			"`]",
			$(
				"(",
				$($doc_link_to)*,
				")",
			)?
			" for more details on the underlying function."
		)]
		$item
	};

	{
		@helper doc_impl unsafe
		{ $(#[$before_meta:meta])* }
		doc { $($doc:tt)* }
		$(doc_link_to { $($doc_link_to:tt)* })?
		item $item:item
	} => {
		$(#[$before_meta])*
		#[doc = ""]
		#[doc = "# Safety"]
		#[doc = ""]
		#[doc = concat!(
			"You must uphold safety invariants of [`",
			$($doc)*,
			"`]",
			$(
				"(",
				$($doc_link_to)*,
				")",
			)?
			"."
		)]
		#[doc = ""]
		#[doc = concat!(
			"See documentation for [`",
			$($doc)*,
			"`]",
			$(
				"(",
				$($doc_link_to)*,
				")",
			)?
			" for more details on the underlying function."
		)]
		$item
	};

	{ @helper rest $self:ident $inner:ident { $($impl:tt)*} } => {
		let $inner = <Self as $crate::ChainConversions>::as_inner_mut(&mut $self);
		let _: () = { $($impl)* };
		$self
	};

	{ @helper rest $self:ident $inner:ident $type:ty { $($impl:tt)*} } => {
		// shushes the unused_mut warning
		// I could modify the macro more to not emit `mut self` in the parameter
		// if it isn't needed, but... this is simpler in code I have to type, and
		// compiler is surely smart enough to remove this
		let _ = &mut $self;

		let $inner = $self.into_inner();
		let inner = { $($impl)* };
		$crate::Chain::from_inner(inner)
	};

	{ @helper return_type } => { Self };
	{ @helper return_type $type:ty } => { $crate::Chain<$type> };

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
