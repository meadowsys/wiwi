crate::decl_chain! {
	struct StringChain;
	inner String;
}

crate::decl_chain! {
	struct StringChainMut['h];
	impl['h] StringChainMut<'h>;
	inner &'h mut String;
}
