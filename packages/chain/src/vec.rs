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
