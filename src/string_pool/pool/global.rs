use crate::LazyWrap;
use super::{ Pool, SlicesWrap };
use ::hashbrown::{ Equivalent, HashSet };
use ::parking_lot::RwLock;
use ::std::hash::{ Hash, Hasher };
use ::std::sync::Arc;

/// The default, global string pool
#[derive(Clone, Debug)]
pub struct GlobalPool;

// #[cfg(test)]
// #[path = "../tests/global_pool.rs"]
// mod tests;

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

	fn raw_to_slice<'r>(&self, raw: &'r Self::Raw) -> &'r [u8] {
		&raw.0
	}

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
				(Some(a), Some(b)) => { return false }

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
