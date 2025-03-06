crate::decl_chain! {
	struct VecChain[T];
	impl[T] VecChain<T>;
	inner Vec<T>;
}

crate::decl_chain! {
	struct VecChainMut['h, T];
	impl['h, T] VecChainMut<'h, T>;
	inner &'h mut Vec<T>;
}

crate::impl_chain_conversions! {
	impl chain [T] VecChain<T>;
	impl chain_mut ['h, T] VecChainMut<'h, T>;
	impl inner [T] Vec<T>;
	type inner Vec<T>;
	type mut_chain VecChainMut<'mut_chain, T>;
}

crate::chain_fns! {
	impl chain [T] VecChain<T>;
	impl chain_mut ['h, T] VecChainMut<'h, T>;
}
