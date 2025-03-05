crate::decl_chain! {
	struct VecChain[T];
	impl[T] VecChain<T>;
	inner Vec<T>;
}
