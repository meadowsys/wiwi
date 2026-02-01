// todo use our own rc when we get it back? we'd want it to respect allocator anyways

use crate::DefaultHashBuilder;

use allocator_api2::alloc::{ Allocator, Global };
use core::cell::UnsafeCell;
use core::iter::FusedIterator;
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

	// todo
	// retain

	// todo
	// extract_if

	#[inline]
	pub fn clear(&mut self) {
		self.keys.clear();
		self.values.clear();
	}

	#[inline]
	pub fn into_keys(self) -> IntoKeys<K, V, A> {
		IntoKeys { inner: self.keys.into_keys() }
	}

	#[inline]
	pub fn into_values(self) -> IntoValues<V, A> {
		IntoValues { inner: self.values.into_iter() }
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

// todo thread safety traits
pub struct Keys<'h, K, V> {
	inner: hashbrown::hash_map::Keys<'h, K, Rc<UnsafeCell<V>>>
}

// todo impl Clone for Keys
// todo impl Debug for Keys
// todo impl Default for Keys

impl<'h, K, V> Iterator for Keys<'h, K, V> {
	type Item = &'h K;

	#[inline]
	fn next(&mut self) -> Option<&'h K> {
		self.inner.next()
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	// hashbrown has a specialised impl
	#[inline]
	fn fold<B, F>(self, init: B, f: F) -> B
	where
		Self: Sized,
		F: FnMut(B, &'h K) -> B
	{
		self.inner.fold(init, f)
	}
}

impl<'h, K, V> ExactSizeIterator for Keys<'h, K, V> {
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<'h, K, V> FusedIterator for Keys<'h, K, V> {}

// todo thread safety traits
pub struct Values<'h, V> {
	inner: hashbrown::hash_set::Iter<'h, Rc<UnsafeCell<V>>>
}

// todo impl Clone for Values
// todo impl Debug for Values
// todo impl Default for Values

impl<'h, V> Iterator for Values<'h, V> {
	type Item = &'h V;

	#[inline ]
	fn next(&mut self) -> Option<&'h V> {
		self.inner.next().map(|next| {
			// SAFETY: we have immutable borrow
			unsafe { &*next.get() }
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	// hashbrown has a specialised impl
	#[inline]
	fn fold<B, F>(self, init: B, mut f: F) -> B
	where
		Self: Sized,
		F: FnMut(B, &'h V) -> B
	{
		self.inner.fold(init, |acc, curr| {
			// SAFETY: we have immutable borrow
			let curr = unsafe { &*curr.get() };

			f(acc, curr)
		})
	}
}

impl<'h, V> ExactSizeIterator for Values<'h, V> {
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<'h, V> FusedIterator for Values<'h, V> {}

// todo
// todo thread safety traits
// pub struct ValuesMut<'h, V> {
// 	inner: hashbrown::hash_set::IntoIter<V>
// }

pub struct Iter<'h, K, V> {
	inner: hashbrown::hash_map::Iter<'h, K, Rc<UnsafeCell<V>>>
}

// todo impl Clone for Iter
// todo impl Debug for Iter
// todo impl Default for Iter

impl<'h, K, V> Iterator for Iter<'h, K, V> {
	type Item = (&'h K, &'h V);

	#[inline]
	fn next(&mut self) -> Option<(&'h K, &'h V)> {
		self.inner.next().map(|(k, v)| {
			// SAFETY: we have immutable borrow
			let v = unsafe { &*v.get() };

			(k, v)
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	#[inline]
	fn fold<B, F>(self, init: B, mut f: F) -> B
	where
		Self: Sized,
		F: FnMut(B, (&'h K, &'h V)) -> B
	{
		self.inner.fold(init, |acc, (k, v)| {
			// SAFETY: we have immutable borrow
			let v = unsafe { &*v.get() };

			f(acc, (k, v))
		})
	}
}

impl<'h, K, V> ExactSizeIterator for Iter<'h, K, V> {
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<'h, K, V> FusedIterator for Iter<'h, K, V> {}

pub struct IntoKeys<K, V, A = Global>
where
	A: Allocator
{
	inner: hashbrown::hash_map::IntoKeys<K, Rc<UnsafeCell<V>>, A>
}

// todo impl Debug for IntoKeys
// todo impl Default for IntoKeys

impl<K, V, A> Iterator for IntoKeys<K, V, A>
where
	A: Allocator
{
	type Item = K;

	#[inline]
	fn next(&mut self) -> Option<K> {
		self.inner.next()
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	#[inline]
	fn fold<B, F>(self, init: B, f: F) -> B
	where
		Self: Sized,
		F: FnMut(B, K) -> B
	{
		self.inner.fold(init, f)
	}
}

impl<K, V, A> ExactSizeIterator for IntoKeys<K, V, A>
where
	A: Allocator
{
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<K, V, A> FusedIterator for IntoKeys<K, V, A>
where
	A: Allocator
{}

pub struct IntoValues<V, A = Global>
where
	A: Allocator
{
	inner: hashbrown::hash_set::IntoIter<Rc<UnsafeCell<V>>, A>
}

// todo impl Debug for IntoValues
// todo impl Default for IntoValues
// todo impl ExactSizeIterator for IntoValues
// todo impl FusedIterator for IntoValues

impl<V, A> Iterator for IntoValues<V, A>
where
	A: Allocator
{
	type Item = V;

	#[inline]
	fn next(&mut self) -> Option<V> {
		self.inner.next().map(|value| {
			debug_assert_eq!(Rc::strong_count(&value), 1);

			// SAFETY: we should have the only strong reference, as
			// the keys map has already been dropped
			let value = unsafe { Rc::try_unwrap(value).unwrap_unchecked() };

			value.into_inner()
		})
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	#[inline]
	fn fold<B, F>(self, init: B, mut f: F) -> B
	where
		Self: Sized,
		F: FnMut(B, V) -> B
	{
		self.inner.fold(init, |acc, curr| {
			debug_assert_eq!(Rc::strong_count(&curr), 1);

			// SAFETY: we should have the only strong reference, as
			// the keys map has already been dropped
			let curr = unsafe { Rc::try_unwrap(curr).unwrap_unchecked() };

			f(acc, curr.into_inner())
		})
	}
}

impl<V, A> ExactSizeIterator for IntoValues<V, A>
where
	A: Allocator
{
	#[inline]
	fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<V, A> FusedIterator for IntoValues<V, A>
where
	A: Allocator
{}
