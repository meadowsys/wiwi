use crate::prelude::*;
use crate::macro_util::void;

/// Generic (works with all types) chaining wrapper
///
/// This wrapper's API is very very simple, consisting of just 4 functions:
///
/// - [`new`](GenericChain::new) creates a new generic chainer (obviously :p)
/// - [`map`](GenericChain::map) takes a closure that gets ownership of the inner
///   value, and returns a new value, possibly of a different type
/// - [`with_inner`](GenericChain::with_inner) takes a closure that can operate
///   on a passed-in mutable reference to the inner value
/// - [`into_inner`](GenericChain::into_inner) will unwrap the struct and return
///   the (possibly modified) contained value.
///
/// The idea is to provide a generic type that can give very basic and generic
/// chaining abilities, useful for types that don't have their own dedicated
/// chainer.
///
/// This type also does not implement any traits, not even the chain ones
/// provided by this module. The API really is just those 4 functions.
///
/// # Examples
///
/// Let's pretend [`VecChain`](super::VecChain) doesn't exist just for a moment...
///
/// ```
/// # use wiwi::chain::GenericChain;
/// // create the chain ...
/// let numbers = GenericChain::new(Vec::<i32>::new())
///    // ... chain push some numbers ...
///    .with_inner(|v| v.push(1))
///    .with_inner(|v| v.push(2))
///    .with_inner(|v| v.push(3))
///    .with_inner(|v| v.push(4))
///    .with_inner(|v| v.push(5))
///    // ... get back the inner vec, now with the pushed elements
///    .into_inner();
///
/// assert_eq!(&*numbers, &[1, 2, 3, 4, 5]);
/// ```
#[must_use = "a chain always takes ownership of itself, performs the operation, then returns itself again"]
#[repr(transparent)]
pub struct GenericChain<T> {
	inner: T
}

impl<T> GenericChain<T> {
	#[inline]
	pub fn new(value: T) -> Self {
		Self { inner: value }
	}

	#[inline]
	pub fn map<T2>(self, f: impl FnOnce(T) -> T2) -> GenericChain<T2> {
		GenericChain::new(f(self.into_inner()))
	}

	#[inline]
	pub fn with_inner<F, Void>(mut self, f: F) -> Self
	where
		F: FnOnce(&mut T) -> Void
	{
		void!(f(&mut self.inner));
		self
	}

	#[inline]
	pub fn into_inner(self) -> T {
		self.inner
	}
}

/// Chaining API to convert any ([`Sized`]) type to [`GenericChain<T>`] (`value.into_generic_chain()`)
pub trait GenericChainConversion: Sized {
	#[inline]
	fn into_generic_chain(self) -> GenericChain<Self> {
		GenericChain::new(self)
	}
}

impl<T> GenericChainConversion for T {}
