use crate::prelude_internal::*;

impl_chain_conversions! { [K, V, S] std::collections::HashMap<K, V, S> }
impl_chain_conversions! {
	#[cfg(feature = "hashbrown")]
	[K, V, S] hashbrown::HashMap<K, V, S>
}

chain_fns! {
	impl and_mut [K, V, S] std::collections::HashMap<K, V, S>;
	#[cfg(feature = "hashbrown")]
	impl and_mut [K, V, S] hashbrown::HashMap<K, V, S>;

	doc "awa"
	fn test_todo_remove_me_lol(inner) {
		let _ = inner;
	}
}
