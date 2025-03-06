crate::decl_chain! {
	struct ArrayChain[T, const N: usize];
	impl[T, const N: usize] ArrayChain<T, N>;
	inner [T; N];
}

crate::decl_chain! {
	struct ArrayMutChain['h, T, const N: usize];
	impl['h, T, const N: usize] ArrayMutChain<'h, T, N>;
	inner &'h mut [T; N];
}

crate::impl_chain_conversions! {
	impl chain [T, const N: usize] ArrayChain<T, N>;
	impl chain_mut ['h, T, const N: usize] ArrayMutChain<'h, T, N>;
	impl inner [T, const N: usize] [T; N];
	type inner [T; N];
	type mut_chain ArrayMutChain<'mut_chain, T, N>;
}
