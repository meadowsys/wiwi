crate::decl_chain! {
	struct ArrayChain[T, const N: usize];
	impl[T, const N: usize] ArrayChain<T, N>;
	inner [T; N];
}

crate::decl_chain! {
	struct ArrayChainMut['h, T, const N: usize];
	impl['h, T, const N: usize] ArrayChainMut<'h, T, N>;
	inner &'h mut [T; N];
}
