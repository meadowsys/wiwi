use std::collections::HashMap;
use std::hash::RandomState;

#[repr(transparent)]
pub struct HashMapChain<K, V, S = RandomState> {
	inner: HashMap<K, V, S>
}

// TODO: eventually ref/mut versions

impl<K, V> HashMapChain<K, V> {
	pub fn new() -> Self {
		HashMap::new().into()
	}

	pub fn with_capacity(capacity: usize) -> Self {
		HashMap::with_capacity(capacity).into()
	}
}

impl<K, V, S> From<HashMap<K, V, S>> for HashMapChain<K, V, S> {
	fn from(value: HashMap<K, V, S>) -> Self {
		Self { inner: value }
	}
}
