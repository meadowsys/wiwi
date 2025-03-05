crate::decl_chain! {
	struct StringChain;
	inner String;
}

crate::decl_chain! {
	struct StringChainMut['h];
	impl['h] StringChainMut<'h>;
	inner &'h mut String;
}

crate::impl_chain_conversions! {
	impl chain [] StringChain;
	impl chain_mut ['h] StringChainMut<'h>;
	impl inner [] String;
	type inner String;
	type mut_chain StringChainMut<'mut_chain>;
}
