// todo use our own rc when we get it back? we'd want it to respect allocator anyways

use crate::DefaultHashBuilder;

use allocator_api2::alloc::{ Allocator, Global };
use hashbrown::{ HashMap, HashSet };
use std::rc::Rc;

pub struct PlaceholderMap<K, V, S = DefaultHashBuilder, A = Global>
where
	A: Allocator
{
	keys: HashMap<K, Rc<V>, S, A>,
	values: HashSet<Rc<V>, S, A>
}

impl<K, V> PlaceholderMap<K, V> {
	#[inline]
	pub fn new() -> Self {
		Self {
			keys: HashMap::with_hasher(DefaultHashBuilder::new()),
			values: HashSet::with_hasher(DefaultHashBuilder::new())
		}
	}

	#[inline]
	pub fn with_key_capacity(capacity: usize) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(capacity, DefaultHashBuilder::new()),
			values: HashSet::with_hasher(DefaultHashBuilder::new())
		}
	}

	#[inline]
	pub fn with_value_capacity(capacity: usize) -> Self {
		Self {
			keys: HashMap::with_hasher(DefaultHashBuilder::new()),
			values: HashSet::with_capacity_and_hasher(capacity, DefaultHashBuilder::new())
		}
	}

	#[inline]
	pub fn with_key_value_capacity(key_capacity: usize, value_capacity: usize) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(key_capacity, DefaultHashBuilder::new()),
			values: HashSet::with_capacity_and_hasher(value_capacity, DefaultHashBuilder::new())
		}
	}
}

impl<K, V, A> PlaceholderMap<K, V, DefaultHashBuilder, A>
where
	A: Allocator + Clone
{
	#[inline]
	pub fn new_in(alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(DefaultHashBuilder::new(), alloc.clone()),
			values: HashSet::with_hasher_in(DefaultHashBuilder::new(), alloc)
		}
	}

	#[inline]
	pub fn with_key_capacity_in(capacity: usize, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), alloc.clone()),
			values: HashSet::with_hasher_in(DefaultHashBuilder::new(), alloc)
		}
	}

	#[inline]
	pub fn with_value_capacity_in(capacity: usize, alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(DefaultHashBuilder::new(), alloc.clone()),
			values: HashSet::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), alloc)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_in(key_capacity: usize, value_capacity: usize, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(key_capacity, DefaultHashBuilder::new(), alloc.clone()),
			values: HashSet::with_capacity_and_hasher_in(value_capacity, DefaultHashBuilder::new(), alloc)
		}
	}
}

impl<K, V, A> PlaceholderMap<K, V, DefaultHashBuilder, A>
where
	A: Allocator
{
	#[inline]
	pub fn new_in2(keys_alloc: A, values_alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(DefaultHashBuilder::new(), keys_alloc),
			values: HashSet::with_hasher_in(DefaultHashBuilder::new(), values_alloc)
		}
	}

	#[inline]
	pub fn with_key_capacity_in2(capacity: usize, keys_alloc: A, values_alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), keys_alloc),
			values: HashSet::with_hasher_in(DefaultHashBuilder::new(), values_alloc)
		}
	}

	#[inline]
	pub fn with_value_capacity_in2(capacity: usize, keys_alloc: A, values_alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(DefaultHashBuilder::new(), keys_alloc),
			values: HashSet::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), values_alloc)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_in2(key_capacity: usize, value_capacity: usize, keys_alloc: A, values_alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(key_capacity, DefaultHashBuilder::new(), keys_alloc),
			values: HashSet::with_capacity_and_hasher_in(value_capacity, DefaultHashBuilder::new(), values_alloc)
		}
	}
}

impl<K, V> Default for PlaceholderMap<K, V> {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

// SAFETY: we have Rc internally, but it is never exposed, so all Rc
// values will get moved at once across thread boundaries
unsafe impl<K, V, S, A> Send for PlaceholderMap<K, V, S, A>
where
	K: Send,
	V: Send,
	S: Send,
	A: Allocator + Send
{}

// going to leave it not Sync for now while I figure out if it is Sync or not
// todo reevaluate this
// unsafe impl<K, V, S, A> Sync for PlaceholderMap<K, V, S, A>
// where
// 	K: Sync,
// 	V: Sync,
// 	S: Sync,
// 	A: Allocator + Sync
// {}
