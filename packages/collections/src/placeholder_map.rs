// todo use our own rc when we get it back? we'd want it to respect allocator anyways

use crate::DefaultHashBuilder;

use allocator_api2::alloc::{ Allocator, Global };
use core::cell::UnsafeCell;
use hashbrown::{ HashMap, HashSet };
use std::rc::Rc;

pub struct PlaceholderMap<K, V, S = DefaultHashBuilder, A = Global>
where
	A: Allocator
{
	keys: HashMap<K, Rc<UnsafeCell<V>>, S, A>,
	values: HashSet<Rc<UnsafeCell<V>>, S, A>
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

impl<K, V, S> PlaceholderMap<K, V, S>
where
	S: Clone
{
	#[inline]
	pub fn with_hasher(hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_hasher(hash_builder.clone()),
			values: HashSet::with_hasher(hash_builder)
		}
	}

	#[inline]
	pub fn with_key_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(capacity, hash_builder.clone()),
			values: HashSet::with_hasher(hash_builder)
		}
	}

	#[inline]
	pub fn with_value_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_hasher(hash_builder.clone()),
			values: HashSet::with_capacity_and_hasher(capacity, hash_builder)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_and_hasher(key_capacity: usize, value_capacity: usize, hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(key_capacity, hash_builder.clone()),
			values: HashSet::with_capacity_and_hasher(value_capacity, hash_builder)
		}
	}
}

impl<K, V, S, A> PlaceholderMap<K, V, S, A>
where
	S: Clone,
	A: Allocator + Clone
{
	#[inline]
	pub fn with_hasher_in(hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(hash_builder.clone(), alloc.clone()),
			values: HashSet::with_hasher_in(hash_builder, alloc)
		}
	}

	#[inline]
	pub fn with_key_capacity_and_hasher_in(capacity: usize, hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(capacity, hash_builder.clone(), alloc.clone()),
			values: HashSet::with_hasher_in(hash_builder, alloc)
		}
	}

	#[inline]
	pub fn with_value_capacity_and_hasher_in(capacity: usize, hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(hash_builder.clone(), alloc.clone()),
			values: HashSet::with_capacity_and_hasher_in(capacity, hash_builder, alloc)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_and_hasher_in(key_capacity: usize, value_capacity: usize, hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(key_capacity, hash_builder.clone(), alloc.clone()),
			values: HashSet::with_capacity_and_hasher_in(value_capacity, hash_builder, alloc)
		}
	}
}

impl<K, V, S, A> PlaceholderMap<K, V, S, A>
where
	A: Allocator
{
	#[inline]
	pub fn key_allocator(&self) -> &A {
		self.keys.allocator()
	}

	#[inline]
	pub fn value_allocator(&self) -> &A {
		self.values.allocator()
	}

	#[inline]
	pub fn key_hasher(&self) -> &S {
		self.keys.hasher()
	}

	#[inline]
	pub fn value_hasher(&self) -> &S {
		self.values.hasher()
	}

	#[inline]
	pub fn key_capacity(&self) -> usize {
		self.keys.capacity()
	}

	#[inline]
	pub fn value_capacity(&self) -> usize {
		self.values.capacity()
	}

	#[inline]
	pub fn keys(&self) -> Keys<'_, K, V> {
		Keys { inner: self.keys.keys() }
	}

	#[inline]
	pub fn values(&self) -> Values<'_, V> {
		Values { inner: self.values.iter() }
	}

	// todo
	// pub fn values_mut(&mut self) -> ValuesMut<'_, V> {}

	#[inline]
	pub fn iter(&self) -> Iter<'_, K, V> {
		Iter { inner: self.keys.iter() }
	}

	// todo
	// pub fn iter_mut(&self) -> Iter<'_, K, V> {}

	#[inline]
	pub fn len(&self) -> usize {
		self.keys.len()
	}

	#[inline]
	pub fn values_len(&self) -> usize {
		self.values.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		debug_assert_eq!(
			self.keys.is_empty(),
			self.values.is_empty(),
			"failure to keep keys and values in sync"
		);

		self.keys.is_empty()
	}

	// todo
	// pub fn drain(&mut self) -> Drain<'_, K, V, A> {}
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

// todo thread safety traits
pub struct Keys<'h, K, V> {
	inner: hashbrown::hash_map::Keys<'h, K, Rc<UnsafeCell<V>>>
}

// todo thread safety traits
pub struct Values<'h, V> {
	inner: hashbrown::hash_set::Iter<'h, Rc<UnsafeCell<V>>>
}

// todo
// todo thread safety traits
// pub struct ValuesMut<'h, V> {
// 	inner: hashbrown::hash_set::IntoIter<V>
// }

pub struct Iter<'h, K, V> {
	inner: hashbrown::hash_map::Iter<'h, K, Rc<UnsafeCell<V>>>
}
