use crate::prelude::*;
use super::{ Chain as _, ChainInner as _ };

super::decl_chain! {
	generics_decl: [T, const N: usize]
	generics_decl_struct_def: [T, const N: usize]
	generics: [T, N]
	chain: ArrayChain
	inner: [T; N]
}

impl<T, const N: usize> ArrayChain<T, N> {
	#[inline]
	pub fn new_uninit() -> ArrayChain<MaybeUninit<T>, N> {
		// SAFETY: `MaybeUninit` has no initialisation requirement, so
		// `[MaybeUninit<T>; N]` is always valid
		unsafe { ArrayChain::from_inner(MaybeUninit::uninit().assume_init()) }
	}

	#[inline]
	pub fn new_zeroed() -> ArrayChain<MaybeUninit<T>, N> {
		// SAFETY: `MaybeUninit` has no initialisation requirement, so
		// `[MaybeUninit<T>; N]` is always valid
		unsafe { ArrayChain::from_inner(MaybeUninit::zeroed().assume_init()) }
	}
}

impl<T, const N: usize> ArrayChain<MaybeUninit<T>, N> {
	/// Assumes all slots inside the array are initialised according to `T`'s
	/// requirements, and converts into an array of T
	///
	/// Note: this implementation is currently subpar, as it does fully copy `self`
	/// into a new container using `ptr::read`. We have to do this because, at the
	/// time of writing:
	///
	/// - `transmute` is a bit too dumb, and is not able to prove `[T; N]` and
	///   `[MaybeUninit<T>; N]` are guaranteed to be equal sized, even though
	///   we can see and prove it
	/// - `transmute_unchecked` is like `transmute` but without that compile time
	///   size check, but it is unstable, and according to a code comment will
	///   almost certainly never be stabilised (reasoning is that it's too unsafe,
	///   too much power to give users :p, and to hopefully find other methods for
	///   doing things without it so that it isn't needed)
	/// - `MaybeUninit::array_assume_init` is unstable (it internally makes use of
	///   `transmute_unchecked`)
	///
	/// We don't know of any other option than to perform a ptr cast,
	/// then read from it.
	///
	/// # Safety
	///
	/// All slots in `self` must be fully initialised with valid values of `T`.
	#[inline]
	pub unsafe fn assume_init(self) -> ArrayChain<T, N> {
		#[expect(clippy::as_conversions, reason = "ptr cast")]
		let ptr = self.as_inner() as *const [MaybeUninit<T>; N] as *const [T; N];

		// SAFETY: `ptr` is obtained from `self`, performed valid cast from array
		// of MaybeUninit to array of T, and caller promises that all slots in the
		// array are properly initialised values of `T`. Also see doc comment on
		// this function for why this is a ptr read rather than a transmute
		unsafe { ptr.read().into_chain() }
	}
}
