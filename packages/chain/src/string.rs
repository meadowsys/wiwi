crate::decl_chain! {
	struct StringChain;
	inner String;
}

crate::decl_chain! {
	struct StringMutChain['h];
	impl['h] StringMutChain<'h>;
	inner &'h mut String;
}

crate::impl_chain_conversions! {
	impl chain [] StringChain;
	impl chain_mut ['h] StringMutChain<'h>;
	impl inner [] String;
	type inner String;
	type mut_chain StringMutChain<'mut_chain>;
}

crate::chain_fns! {
	impl chain [] StringChain;
	impl chain_mut ['h] StringMutChain<'h>;
}
