use super::{ ChainConversions, Output };
use core::cmp;

crate::decl_chain! {
	struct VecChain[T];
	impl[T] VecChain<T>;
	inner Vec<T>;
}

crate::decl_chain! {
	struct VecMutChain['h, T];
	impl['h, T] VecMutChain<'h, T>;
	inner &'h mut Vec<T>;
}

crate::impl_chain_conversions! {
	impl chain [T] VecChain<T>;
	impl chain_mut ['h, T] VecMutChain<'h, T>;
	impl inner [T] Vec<T>;
	type inner Vec<T>;
	type mut_chain VecMutChain<'mut_chain, T>;
}

crate::chain_fns! {
	impl chain [T] VecChain<T>;
	impl chain_mut ['h, T] VecMutChain<'h, T>;

	fn append(
		inner,
		other: &mut impl ChainConversions<Inner = Vec<T>>
	) => inner.append(other.as_inner_mut());

	fn binary_search(
		inner,
		x: &T,
		out: impl Output<Result<usize, usize>>
	) where {
		T: Ord
	} => out.write(inner.binary_search(x));

	fn binary_search_by(
		inner,
		f: impl FnMut(&T) -> cmp::Ordering,
		out: impl Output<Result<usize, usize>>
	) => out.write(inner.binary_search_by(f));

	fn binary_search_by_key[B](
		inner,
		b: &B,
		f: impl FnMut(&T) -> B,
		out: impl Output<Result<usize, usize>>
	) where {
		B: Ord
	} => out.write(inner.binary_search_by_key(b, f));

	fn capacity(
		inner,
		out: impl Output<usize>
	) => out.write(inner.capacity());
}
