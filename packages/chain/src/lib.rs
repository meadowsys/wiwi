use self::sealed::Sealed;

pub trait Chain: Sized + Sealed {
	type Inner: ChainInner<Chain = Self>;

	fn from_inner(inner: Self::Inner) -> Self;

	#[inline]
	fn into_inner(self) -> Self::Inner {
		Self::Inner::from_chain(self)
	}

	fn as_inner(&self) -> &Self::Inner;
	fn as_inner_mut(&mut self) -> &mut Self::Inner;

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
		let _ = f(self.as_inner_mut());
		self
	}
}

pub trait ChainInner: Sized + Sealed {
	type Chain: Chain<Inner = Self>;

	fn from_chain(chain: Self::Chain) -> Self;

	#[inline]
	fn into_chain(self) -> Self::Chain {
		Self::Chain::from_inner(self)
	}
}

macro_rules! decl_chain {
	{
		struct $chain:ident;
		inner $inner:ty;
	} => {
		$crate::decl_chain! {
			struct $chain;
			impl[] $chain;
			inner $inner;
		}
	};

	{
		struct $chain:ty;
		impl[$($chain_impl_generics:tt)*] $chain_impl:ty;
		inner $inner:ty;
	} => {};
}
use decl_chain;

/// notouchie
mod sealed {
	/// notouchie
	pub trait Sealed {}
}
