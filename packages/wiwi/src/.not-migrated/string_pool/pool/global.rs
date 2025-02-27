use crate::lazy_wrap::LazyWrap;
use super::{ Pool, SlicesWrap };
use hashbrown::{ Equivalent, HashSet };
use parking_lot::RwLock;
use std::hash::{ Hash, Hasher };
use std::sync::Arc;

/// The default, global string pool
#[derive(Clone, Debug, Default)]
pub struct GlobalPool;

/// The actual backing store for the default global pool
static POOL: LazyWrap<RwLock<HashSet<<GlobalPool as Pool>::Raw>>> = LazyWrap::new(|| {
	let set = HashSet::new();
	RwLock::new(set)
});

impl Pool for GlobalPool {
	type Raw = Arc<SliceHashWrap>;

	unsafe fn raw_from_slices(&self, slices: SlicesWrap) -> Self::Raw {
		let pool = POOL.read();

		if let Some(raw) = pool.get(&slices) {
			let raw = Arc::clone(raw);
			drop(pool);
			raw
		} else {
			drop(pool);

			let mut pool = POOL.write();
			let raw = pool.get_or_insert_with(&slices, |slices| {
				Arc::new(SliceHashWrap(slices.to_boxed_slice()))
			});

			let raw = Arc::clone(raw);
			drop(pool);
			raw
		}
	}

	#[inline]
	fn raw_to_slice<'r>(&self, raw: &'r Self::Raw) -> &'r [u8] {
		&raw.0
	}

	#[inline]
	fn raw_clone(&self, raw: &Self::Raw) -> Self::Raw {
		Arc::clone(raw)
	}
}

/// Wrapper for `Box<[u8]>` that hashes the slice within by repeatedly
/// calling `Hasher::write_u8`, matching [`Hash`] impl of [`SlicesWrap`]
#[derive(Debug)]
#[repr(transparent)]
pub struct SliceHashWrap(Box<[u8]>);

impl Hash for SliceHashWrap {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.iter().copied()
			.for_each(|b| state.write_u8(b));
	}
}

impl PartialEq for SliceHashWrap {
	fn eq(&self, other: &Self) -> bool {
		*self.0 == *other.0
	}
}

impl Eq for SliceHashWrap {}

impl<'h> Equivalent<<GlobalPool as Pool>::Raw> for SlicesWrap<'h> {
	fn equivalent(&self, key: &<GlobalPool as Pool>::Raw) -> bool {
		let mut iter1 = key.0.iter().copied();
		let mut iter2 = self.into_iter();

		loop {
			match (iter1.next(), iter2.next()) {
				// Some Some
				//    - if a == b, good but not done yet, continue
				(Some(a), Some(b)) if a == b => { continue }
				//    - else (a != b), not good, return false
				(Some(_), Some(_)) => { return false }

				// Some None
				// None Some
				//    - iters are of different lengths
				//    - return false
				(Some(_), None) | (None, Some(_)) => { return false }

				// None None
				//    - if we haven't returned yet, all bytes in iter are equal
				//    - both iters are also same length
				//    - return true
				(None, None) => { return true }
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use hashbrown::hash_map::DefaultHashBuilder;
	use rand::{ Rng, thread_rng };
	use std::string::String as StdString;
	use std::iter::repeat;
	use std::hash::BuildHasher;

	#[test]
	fn slices_wrap_iter_hash_and_eq() {
		let mut rng = thread_rng();
		let hash_builder = hashbrown::hash_map::DefaultHashBuilder::default();

		for _ in 0..1000 {
			// generate vec of random length 0-10, with strings 0-100 chars
			let strs = repeat(0u8)
				.take(rng.gen_range(1..20))
				.map(|_| rand_std_string())
				.collect::<Vec<_>>();

			// create instance of SliceHashWrap (joining strings)
			let pool_strs = strs.iter()
				.map(|s| &**s)
				.collect::<String>();
			let pool_strs = Arc::new(SliceHashWrap(pool_strs.into_bytes().into_boxed_slice()));

			// create instance of SlicesWrap
			let mut _slices = strs.iter()
				.map(|s| s.as_bytes())
				.collect::<Vec<_>>();
			let slices = SlicesWrap(&_slices);

			let hash_pool = hash_item(&hash_builder, &pool_strs);
			let hash_slices = hash_item(&hash_builder, &slices);

			// test hash eq
			assert_eq!(hash_pool, hash_slices, "hashes should be equal");
			// test actual eq
			assert!(slices.equivalent(&pool_strs), "pool and slices should be equal");

			// this one is guaranteed by rand_std_string generating 1..100 chars
			// panics when that is changed to 0..100
			// TODO: refactor to make it be able to generate 0..100 without issue
			let last = _slices.last_mut().unwrap();
			let last_str = unsafe { std::str::from_utf8_unchecked(last) };
			*last = &last[..last.len() - last_str.chars().last().unwrap().len_utf8()];

			let slices = SlicesWrap(&_slices);
			let hash_slices = hash_item(&hash_builder, &slices);

			assert_ne!(hash_pool, hash_slices, "hashes should not be eq");
			assert!(!slices.equivalent(&pool_strs), "pool and slices should not be equal");
		}
	}

	fn rand_std_string() -> StdString {
		let mut rng = thread_rng();
		let mut vec = vec![' '; rng.gen_range(1..100)];
		rng.fill(&mut *vec);
		vec.into_iter().collect()
	}

	fn hash_item<T: Hash>(hash_builder: &DefaultHashBuilder, item: &T) -> u64 {
		let mut hasher = hash_builder.build_hasher();
		item.hash(&mut hasher);
		hasher.finish()
	}
}
