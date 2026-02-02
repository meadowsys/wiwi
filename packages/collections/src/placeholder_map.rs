// todo use our own rc when we get it back? we'd want it to respect allocator anyways

use crate::DefaultHashBuilder;

use self::rc_mut::RcMut;

use allocator_api2::alloc::{ Allocator, Global };
use core::hash::{ BuildHasher, Hash, Hasher };
use core::iter::FusedIterator;
use hashbrown::HashMap;

// todo we should switch to an api of our own making at some point
pub use hashbrown::Equivalent;

pub struct PlaceholderMap<K, V, S = DefaultHashBuilder, A = Global>
where
	A: Allocator
{
	keys: HashMap<K, RcMut<V>, S, A>,
	values: HashMap<RcMut<V>, (), S, A>
}

impl<K, V> PlaceholderMap<K, V> {
	#[inline]
	pub fn new() -> Self {
		Self {
			keys: HashMap::with_hasher(DefaultHashBuilder::new()),
			values: HashMap::with_hasher(DefaultHashBuilder::new())
		}
	}

	#[inline]
	pub fn with_key_capacity(capacity: usize) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(capacity, DefaultHashBuilder::new()),
			values: HashMap::with_hasher(DefaultHashBuilder::new())
		}
	}

	#[inline]
	pub fn with_value_capacity(capacity: usize) -> Self {
		Self {
			keys: HashMap::with_hasher(DefaultHashBuilder::new()),
			values: HashMap::with_capacity_and_hasher(capacity, DefaultHashBuilder::new())
		}
	}

	#[inline]
	pub fn with_key_value_capacity(key_capacity: usize, value_capacity: usize) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(key_capacity, DefaultHashBuilder::new()),
			values: HashMap::with_capacity_and_hasher(value_capacity, DefaultHashBuilder::new())
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
			values: HashMap::with_hasher_in(DefaultHashBuilder::new(), alloc)
		}
	}

	#[inline]
	pub fn with_key_capacity_in(capacity: usize, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), alloc.clone()),
			values: HashMap::with_hasher_in(DefaultHashBuilder::new(), alloc)
		}
	}

	#[inline]
	pub fn with_value_capacity_in(capacity: usize, alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(DefaultHashBuilder::new(), alloc.clone()),
			values: HashMap::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), alloc)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_in(key_capacity: usize, value_capacity: usize, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(key_capacity, DefaultHashBuilder::new(), alloc.clone()),
			values: HashMap::with_capacity_and_hasher_in(value_capacity, DefaultHashBuilder::new(), alloc)
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
			values: HashMap::with_hasher(hash_builder)
		}
	}

	#[inline]
	pub fn with_key_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(capacity, hash_builder.clone()),
			values: HashMap::with_hasher(hash_builder)
		}
	}

	#[inline]
	pub fn with_value_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_hasher(hash_builder.clone()),
			values: HashMap::with_capacity_and_hasher(capacity, hash_builder)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_and_hasher(key_capacity: usize, value_capacity: usize, hash_builder: S) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher(key_capacity, hash_builder.clone()),
			values: HashMap::with_capacity_and_hasher(value_capacity, hash_builder)
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
			values: HashMap::with_hasher_in(hash_builder, alloc)
		}
	}

	#[inline]
	pub fn with_key_capacity_and_hasher_in(capacity: usize, hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(capacity, hash_builder.clone(), alloc.clone()),
			values: HashMap::with_hasher_in(hash_builder, alloc)
		}
	}

	#[inline]
	pub fn with_value_capacity_and_hasher_in(capacity: usize, hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_hasher_in(hash_builder.clone(), alloc.clone()),
			values: HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc)
		}
	}

	#[inline]
	pub fn with_key_value_capacity_and_hasher_in(key_capacity: usize, value_capacity: usize, hash_builder: S, alloc: A) -> Self {
		Self {
			keys: HashMap::with_capacity_and_hasher_in(key_capacity, hash_builder.clone(), alloc.clone()),
			values: HashMap::with_capacity_and_hasher_in(value_capacity, hash_builder, alloc)
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
		Values { inner: self.values.keys() }
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
		IntoValues { inner: self.values.into_keys() }
	}
}

impl<K, V, S, A> PlaceholderMap<K, V, S, A>
where
	K: Eq + Hash,
	S: BuildHasher,
	A: Allocator
{
	#[inline]
	pub fn reserve_keys(&mut self, additional: usize) {
		self.keys.reserve(additional);
	}

	#[inline]
	pub fn try_reserve_keys(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.keys.try_reserve(additional)
			.map_err(TryReserveError::from_hashbrown)
	}

	#[inline]
	pub fn shrink_keys_to(&mut self, min_capacity: usize) {
		self.keys.shrink_to(min_capacity);
	}

	#[inline]
	pub fn shrink_keys_to_fit(&mut self) {
		self.keys.shrink_to_fit();
	}

	#[inline]
	pub fn get<Q>(&self, k: &Q) -> Option<&V>
	where
		Q: Hash + Equivalent<K> + ?Sized
	{
		self.keys.get(k).map(|v| {
			// SAFETY: we have shared borrow over the entire struct
			unsafe { v.as_ref() }
		})
	}

	#[inline]
	pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
	where
		Q: Hash + Equivalent<K> + ?Sized
	{
		self.keys.get_key_value(k).map(|(k, v)| {
			// SAFETY: we have shared borrow over the entire struct
			let v = unsafe { v.as_ref() };

			(k, v)
		})
	}

	// todo get_key_value_mut

	#[inline]
	pub fn contains_key<Q>(&self, k: &Q) -> bool
	where
		Q: Hash + Equivalent<K> + ?Sized
	{
		self.keys.contains_key(k)
	}

	// todo get_mut
	// todo get_disjoint_mut
	// todo get_disjoint_unchecked_mut
	// todo get_disjoint_key_value_mut
	// todo get_disjoint_key_value_unchecked_mut
}

impl<K, V, S, A> PlaceholderMap<K, V, S, A>
where
	V: Eq + Hash,
	S: BuildHasher,
	A: Allocator
{
	#[inline]
	pub fn reserve_values(&mut self, additional: usize) {
		self.values.reserve(additional);
	}

	#[inline]
	pub fn try_reserve_values(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.values.try_reserve(additional)
			.map_err(TryReserveError::from_hashbrown)
	}

	#[inline]
	pub fn shrink_values_to(&mut self, min_capacity: usize) {
		self.values.shrink_to(min_capacity);
	}

	#[inline]
	pub fn shrink_values_to_fit(&mut self) {
		self.values.shrink_to_fit();
	}
}

impl<K, V, S, A> PlaceholderMap<K, V, S, A>
where
	K: Eq + Hash,
	V: Eq + Hash,
	S: BuildHasher,
	A: Allocator
{
	#[inline]
	pub fn reserve_keys_values(&mut self, additional_keys: usize, additional_values: usize) {
		self.reserve_keys(additional_keys);
		self.reserve_values(additional_values);
	}

	#[inline]
	pub fn try_reserve_keys_values(
		&mut self,
		additional_keys: usize,
		additional_values: usize
	) -> Result<(), TryReserveKeysValuesError> {
		let mut ok = true;
		let mut combined_error = TryReserveKeysValuesError {
			keys: None,
			values: None
		};

		if let Err(error) = self.try_reserve_keys(additional_keys) {
			ok = false;
			combined_error.keys = Some(error);
		}

		if let Err(error) = self.try_reserve_values(additional_values) {
			ok = false;
			combined_error.values = Some(error);
		}

		if ok {
			Ok(())
		} else {
			Err(combined_error)
		}
	}

	#[inline]
	pub fn shrink_keys_values_to(&mut self, min_capacity_keys: usize, min_capacity_values: usize) {
		self.shrink_keys_to(min_capacity_keys);
		self.shrink_values_to(min_capacity_values);
	}

	#[inline]
	pub fn shrink_keys_values_to_fit(&mut self) {
		self.shrink_keys_to_fit();
		self.shrink_values_to_fit();
	}

	#[inline]
	pub fn insert(&mut self, k: K, v: V) -> &V {
		let v = self.keys
			.entry(k)
			.or_insert_with(|| {
				let (v, _) = get_or_insert_value(&mut self.values, v);
				v
			});

		// SAFETY: we have unique borrow over the entire struct
		unsafe { v.as_ref() }
	}

	/// # Safety
	///
	/// See safety docs of [`hashbrown::HashMap::insert_unique_unchecked`]
	// todo the original api returned (&K, &mut V)
	#[inline]
	pub unsafe fn insert_unique_unchecked(&mut self, k: K, v: V) -> (&K, &V) {
		let (v, _) = get_or_insert_value(&mut self.values, v);

		// SAFETY: caller promises to uphold the safety invariant of this
		let (k, v) = unsafe { self.keys.insert_unique_unchecked(k, v) };

		// SAFETY: we have unique borrow over the entire struct
		let v = unsafe { v.as_ref() };

		(k, v)
	}

	// todo the original api returned Result<&mut V, OccupiedError<'_, K, V, S, A>>
	// todo maybe we should have our own error type here
	#[expect(clippy::result_unit_err, reason = "we'll get there eventually")]
	#[inline]
	pub fn try_insert(&mut self, k: K, v: V) -> Result<&V, ()> {
		let (v, is_new) = get_or_insert_value(&mut self.values, v);

		match self.keys.try_insert(k, v) {
			Ok(v) => {
				// SAFETY: we have unique borrow over the entire struct
				Ok(unsafe { v.as_ref() })
			}
			Err(err) => {
				if is_new { remove_value(&mut self.values, &err.value) }
				Err(())
			}
		}
	}

	/// Removes a key from the map, returning the value at the key if the key was
	/// the last key to point at the value
	#[inline]
	pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
	where
		Q: Hash + Equivalent<K> + ?Sized
	{
		self.keys.remove(k).and_then(|v| {
			remove_and_unwrap_if_last_value(&mut self.values, v)
		})
	}

	/// Removes a key from the map, returning the stored key if the key was
	/// previously in the map, and the value at the key if the key was the last
	/// key to point at the value
	#[inline]
	pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, Option<V>)>
	where
		Q: Hash + Equivalent<K> + ?Sized
	{
		self.keys.remove_entry(k).map(|(k, v)| {
			let v = remove_and_unwrap_if_last_value(&mut self.values, v);

			(k, v)
		})
	}

	#[inline]
	pub fn allocation_size(&self) -> usize {
		self.keys.allocation_size() + self.values.allocation_size()
	}
}

impl<K, V> Default for PlaceholderMap<K, V> {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

// SAFETY: we have Rc internally, but it is never exposed, so all strong
// references for all values will get moved at once across a thread boundaries
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

/// Utility function to get a Rc value out of values map or insert a new one in,
/// returning the RcMut and a boolean to indicate if the value is newly inserted or not
///
/// This is a free function and not impl on the map itself because we want to
/// only have `&mut self.values` and not the whole struct, and rust doesn't let
/// us do that through the self parameter
#[inline]
fn get_or_insert_value<V, S, A>(
	values: &mut HashMap<RcMut<V>, (), S, A>,
	v: V
) -> (RcMut<V>, bool)
where
	V: Eq + Hash,
	S: BuildHasher,
	A: Allocator
{
	use hashbrown::hash_map::Entry::*;

	// .entry() doesn't even keep the key if the entry already exists
	// so will guarantee we don't introduce multiple copies of the value
	let (v_entry, is_new) = match values.entry(RcMut::new(v)) {
		Occupied(entry) => { (entry, false) }
		Vacant(entry) => { (entry.insert_entry(()), true) }
	};

	(RcMut::clone_rc(v_entry.key()), is_new)
}

/// Utility function to remove a value we are 100% sure already exists in
/// self.values, complete with debug assertion
///
/// This is a free function and not impl on the map itself because we want to
/// only have `&mut self.values` and not the whole struct, and rust doesn't let
/// us do that through the self parameter
fn remove_value<V, S, A>(values: &mut HashMap<RcMut<V>, (), S, A>, v: &RcMut<V>)
where
	V: Eq + Hash,
	S: BuildHasher,
	A: Allocator
{
	let removed = values.remove(v);
	debug_assert!(removed.is_some(), "completely pointless sanity check");
}

/// Checks that the provided RcMut is the last reference within the map, removing
/// it from `self.values` and unwrapping it if so
///
/// A provided value is considered to be the last reference within the map if
/// it has exactly 2 strong references: the one that was just passed in, and
/// the one in self.values. This function additionally assumes that the
/// provided RcMut is present in the provided self.values.
///
/// This is a free function and not impl on the map itself because we want to
/// only have `&mut self.values` and not the whole struct, and rust doesn't let
/// us do that through the self parameter
fn remove_and_unwrap_if_last_value<V, S, A>(values: &mut HashMap<RcMut<V>, (), S, A>, v: RcMut<V>) -> Option<V>
where
	V: Eq + Hash,
	S: BuildHasher,
	A: Allocator
{
	// 2:
	// - the one we own right now (in `v`)
	// - the one in self.values
	(v.strong_count() == 2).then(|| {
		remove_value(values, &v);

		// SAFETY: we had two, and just removed the other one,
		// so we have the last RcMut pointing to this value
		unsafe { v.into_inner_unchecked() }
	})
}

// todo thread safety traits
pub struct Keys<'h, K, V> {
	inner: hashbrown::hash_map::Keys<'h, K, RcMut<V>>
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
	inner: hashbrown::hash_map::Keys<'h, RcMut<V>, ()>
}

// todo impl Clone for Values
// todo impl Debug for Values
// todo impl Default for Values

impl<'h, V> Iterator for Values<'h, V> {
	type Item = &'h V;

	#[inline ]
	fn next(&mut self) -> Option<&'h V> {
		self.inner.next().map(|next| {
			// SAFETY: we have shared borrow over the entire struct
			unsafe { next.as_ref() }
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
			// SAFETY: we have shared borrow over the entire struct
			let curr = unsafe { curr.as_ref() };

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
	inner: hashbrown::hash_map::Iter<'h, K, RcMut<V>>
}

// todo impl Clone for Iter
// todo impl Debug for Iter
// todo impl Default for Iter

impl<'h, K, V> Iterator for Iter<'h, K, V> {
	type Item = (&'h K, &'h V);

	#[inline]
	fn next(&mut self) -> Option<(&'h K, &'h V)> {
		self.inner.next().map(|(k, v)| {
			// SAFETY: we have shared borrow over the entire struct
			let v = unsafe { v.as_ref() };

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
			// SAFETY: we have shared borrow over the entire struct
			let v = unsafe { v.as_ref() };

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
	inner: hashbrown::hash_map::IntoKeys<K, RcMut<V>, A>
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
	inner: hashbrown::hash_map::IntoKeys<RcMut<V>, (), A>
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
			// SAFETY: we should have the only strong reference, as
			// the keys map has already been dropped
			unsafe { value.into_inner_unchecked() }
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
			// SAFETY: we should have the only strong reference, as
			// the keys map has already been dropped
			let curr = unsafe { curr.into_inner_unchecked() };

			f(acc, curr)
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

pub enum TryReserveError {
	CapacityOverflow,
	AllocError {
		layout: std::alloc::Layout
	}
}

impl TryReserveError {
	#[inline]
	fn from_hashbrown(err: hashbrown::TryReserveError) -> Self {
		match err {
			hashbrown::TryReserveError::AllocError { layout } => {
				TryReserveError::AllocError { layout }
			}
			hashbrown::TryReserveError::CapacityOverflow => {
				TryReserveError::CapacityOverflow
			}
		}
	}
}

pub struct TryReserveKeysValuesError {
	pub keys: Option<TryReserveError>,
	pub values: Option<TryReserveError>
}

mod rc_mut {
	use super::*;

	use core::cell::UnsafeCell;
	use std::rc::Rc;

	/// Unsafe `Rc` wrapper that allows unsafe mut access to the value, even
	/// with multiple strong references
	pub(super) struct RcMut<T> {
		inner: Rc<UnsafeCell<T>>
	}

	impl<T> RcMut<T> {
		#[inline]
		pub fn new(value: T) -> Self {
			let value = UnsafeCell::new(value);
			let value = Rc::new(value);
			Self { inner: value }
		}

		#[inline]
		pub fn clone_rc(rc: &Self) -> Self {
			Self { inner: Rc::clone(&rc.inner) }
		}

		/// # Safety
		///
		/// You must ensure no unique references can exist when this is called
		#[inline]
		pub unsafe fn as_ref(&self) -> &T {
			// SAFETY: caller upholds reference aliasing invariant, and
			// ptr is valid because we just got it from an UnsafeCell
			unsafe { &*self.inner.get() }
		}

		/// # Safety
		///
		/// You must ensure no other references, unique or shared, can exist
		/// when this is called
		#[expect(dead_code, reason = "todo keeping for now but idk if i'm actually going to use it")]
		#[inline]
		pub unsafe fn as_mut(&mut self) -> &mut T {
			// SAFETY: caller upholds reference aliasing invariant, and
			// ptr is valid because we just got it from an UnsafeCell
			unsafe { &mut *self.inner.get() }
		}

		#[inline]
		pub fn strong_count(&self) -> usize {
			Rc::strong_count(&self.inner)
		}

		/// # Safety
		///
		/// You must ensure that no other strong references can exist when this is called
		#[inline]
		pub unsafe fn into_inner_unchecked(self) -> T {
			debug_assert_eq!(Rc::strong_count(&self.inner), 1);

			// SAFETY: caller ensures there is only 1 strong reference, so this
			// won't be Err
			let cell = unsafe { Rc::try_unwrap(self.inner).unwrap_unchecked() };

			cell.into_inner()
		}
	}

	impl<T: Hash> Hash for RcMut<T> {
		#[inline]
		fn hash<H: Hasher>(&self, state: &mut H) {
			// SAFETY: assuming this is only used in HashMap
			// while we have only immutable borrows
			let value = unsafe { self.as_ref() };

			T::hash(value, state)
		}
	}

	impl<T: PartialEq> PartialEq for RcMut<T> {
		#[inline]
		fn eq(&self, other: &Self) -> bool {
			// SAFETY: assuming this is only used in HashMap
			// while we have only immutable borrows
			let value_self = unsafe { self.as_ref() };

			// SAFETY: see above
			let value_other = unsafe { other.as_ref() };

			PartialEq::eq(value_self, value_other)
		}

		#[expect(
			clippy::partialeq_ne_impl,
			reason = "T might have overridden it for whatever reason, we should use it"
		)]
		#[inline]
		fn ne(&self, other: &Self) -> bool {
			// SAFETY: assuming this is only used in HashMap
			// while we have only immutable borrows
			let value_self = unsafe { self.as_ref() };

			// SAFETY: see above
			let value_other = unsafe { other.as_ref() };

			PartialEq::ne(value_self, value_other)
		}
	}

	impl<T: Eq> Eq for RcMut<T> {}
}

// todo entry
// todo entry_ref
